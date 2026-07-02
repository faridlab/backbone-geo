//! Golden-case tests for the geo reference hierarchy.
//! Self-contained fixture (own random codes) + a smoke check against the loaded Indonesia seed.
//! Requires DATABASE_URL (defaults to local dev Postgres on :5433).

use sqlx::{PgPool, Row};
use uuid::Uuid;

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/backbone_geo".to_string());
    PgPool::connect(&url).await.unwrap()
}
fn uq(p: &str) -> String {
    format!("{p}-{}", &Uuid::new_v4().simple().to_string()[..8])
}

/// Insert a minimal country→province→city→district→subdistrict chain; return the ids.
async fn fixture(pool: &PgPool) -> (Uuid, Uuid, Uuid, Uuid, Uuid) {
    let c = Uuid::new_v4();
    sqlx::query("INSERT INTO geo.countries (id, isocode, name, phonecode) VALUES ($1,$2,'Testland',62)")
        .bind(c).bind(uq("T")).execute(pool).await.unwrap();
    let p = Uuid::new_v4();
    sqlx::query("INSERT INTO geo.provinces (id, name, country_id) VALUES ($1,'Prov',$2)")
        .bind(p).bind(c).execute(pool).await.unwrap();
    let ci = Uuid::new_v4();
    sqlx::query("INSERT INTO geo.cities (id, name, country_id, province_id) VALUES ($1,'City',$2,$3)")
        .bind(ci).bind(c).bind(p).execute(pool).await.unwrap();
    let d = Uuid::new_v4();
    sqlx::query("INSERT INTO geo.districts (id, name, country_id, province_id, city_id) VALUES ($1,'Dist',$2,$3,$4)")
        .bind(d).bind(c).bind(p).bind(ci).execute(pool).await.unwrap();
    let s = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO geo.subdistricts (id, name, country_id, province_id, city_id, district_id, postal_code, latitude, longitude)
         VALUES ($1,'Sub',$2,$3,$4,$5,'12345',-6.2,106.8)",
    )
    .bind(s).bind(c).bind(p).bind(ci).bind(d).execute(pool).await.unwrap();
    (c, p, ci, d, s)
}

// GGC-1: an address resolves up the full chain subdistrict → district → city → province → country.
#[tokio::test]
async fn address_chain_resolves() {
    let pool = pool().await;
    let (_c, _p, _ci, _d, s) = fixture(&pool).await;
    let row = sqlx::query(
        "SELECT co.name AS country, pr.name AS province, ci.name AS city, di.name AS district,
                sd.name AS subdistrict, sd.postal_code
         FROM geo.subdistricts sd
         JOIN geo.districts di ON di.id = sd.district_id
         JOIN geo.cities ci    ON ci.id = sd.city_id
         JOIN geo.provinces pr ON pr.id = sd.province_id
         JOIN geo.countries co ON co.id = sd.country_id
         WHERE sd.id = $1",
    )
    .bind(s)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.get::<String, _>("country"), "Testland");
    assert_eq!(row.get::<String, _>("province"), "Prov");
    assert_eq!(row.get::<String, _>("district"), "Dist");
    assert_eq!(row.get::<String, _>("subdistrict"), "Sub");
    assert_eq!(row.get::<String, _>("postal_code"), "12345");
}

// GGC-2: subdistrict name is unique within a district.
#[tokio::test]
async fn subdistrict_unique_per_district() {
    let pool = pool().await;
    let (c, p, ci, d, _s) = fixture(&pool).await;
    // "Sub" already exists in district d → inserting it again must violate the unique index.
    let dup = sqlx::query(
        "INSERT INTO geo.subdistricts (id, name, country_id, province_id, city_id, district_id)
         VALUES ($1,'Sub',$2,$3,$4,$5)",
    )
    .bind(Uuid::new_v4()).bind(c).bind(p).bind(ci).bind(d)
    .execute(&pool)
    .await;
    assert!(dup.is_err(), "duplicate (district_id, name) must be rejected");
}

// GGC-3: coordinates round-trip.
#[tokio::test]
async fn coords_stored() {
    let pool = pool().await;
    let (_c, _p, _ci, _d, s) = fixture(&pool).await;
    use rust_decimal::Decimal;
    use std::str::FromStr;
    let (lat, lng): (Option<Decimal>, Option<Decimal>) =
        sqlx::query_as("SELECT latitude, longitude FROM geo.subdistricts WHERE id=$1")
            .bind(s).fetch_one(&pool).await.unwrap();
    assert_eq!(lat.unwrap(), Decimal::from_str("-6.2").unwrap());
    assert_eq!(lng.unwrap(), Decimal::from_str("106.8").unwrap());
}

// GGC-4 (smoke): the loaded Indonesia reference seed is present and well-linked.
// Skips gracefully if the seed hasn't been loaded (fresh CI DB).
#[tokio::test]
async fn indonesia_seed_smoke() {
    let pool = pool().await;
    let provinces: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM geo.provinces WHERE country_id IN (SELECT id FROM geo.countries WHERE isocode='ID')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    if provinces == 0 {
        // Council 2026-07-02: in CI/prod (GEO_REQUIRE_SEED=1) an unseeded geo is a HARD FAILURE,
        // not a silent skip — otherwise a green suite is consistent with an empty production geo.
        if std::env::var("GEO_REQUIRE_SEED").as_deref() == Ok("1") {
            panic!("GEO_REQUIRE_SEED=1 but geo is not seeded — load migrations/seeds/*.sql");
        }
        eprintln!("Indonesia seed not loaded; skipping smoke assertions (set GEO_REQUIRE_SEED=1 to fail)");
        return;
    }
    assert_eq!(provinces, 38, "Indonesia has 38 provinces in the adopted seed");
    let jkt: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM geo.subdistricts s JOIN geo.provinces p ON p.id=s.province_id WHERE p.name='DKI Jakarta'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(jkt > 200, "DKI Jakarta should have >200 subdistricts, got {jkt}");
}

// GGC-5: readiness passes on the seeded DB, and the loaded seed has ZERO ancestor drift.
#[tokio::test]
async fn readiness_and_ancestor_consistency() {
    use backbone_geo::{ancestor_drift_count, geo_readiness_check, GeoReadinessError};
    let pool = pool().await;
    let countries: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM geo.countries")
        .fetch_one(&pool).await.unwrap();
    // The adopted seed must be internally consistent (denormalized ancestors agree with the chain).
    let drift = ancestor_drift_count(&pool).await.unwrap();
    assert_eq!(drift, 0, "loaded seed must have 0 denormalized-ancestor mismatches");

    match geo_readiness_check(&pool).await {
        Ok(()) => assert!(countries > 0),
        Err(GeoReadinessError::NotSeeded) => {
            if std::env::var("GEO_REQUIRE_SEED").as_deref() == Ok("1") {
                panic!("GEO_REQUIRE_SEED=1 but readiness reports NotSeeded");
            }
            assert_eq!(countries, 0);
        }
        Err(e) => panic!("unexpected readiness error: {e}"),
    }
}

// GGC-6: readiness DETECTS ancestor drift (guards the MEDIUM finding) — craft a bad subdistrict.
#[tokio::test]
async fn readiness_detects_ancestor_drift() {
    use backbone_geo::{ancestor_drift_count, geo_readiness_check, GeoReadinessError};
    let pool = pool().await;
    let (c, p, ci, d, _s) = fixture(&pool).await;
    let base = ancestor_drift_count(&pool).await.unwrap();
    // A subdistrict whose province_id points at the WRONG province (a different one we make).
    let bad_prov = Uuid::new_v4();
    sqlx::query("INSERT INTO geo.provinces (id, name, country_id) VALUES ($1,'Wrong',$2)")
        .bind(bad_prov).bind(c).execute(&pool).await.unwrap();
    let bad = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO geo.subdistricts (id, name, country_id, province_id, city_id, district_id)
         VALUES ($1,$2,$3,$4,$5,$6)",
    )
    .bind(bad).bind(uq("BadSub")).bind(c).bind(bad_prov).bind(ci).bind(d) // province_id != district's
    .execute(&pool).await.unwrap();

    let after = ancestor_drift_count(&pool).await.unwrap();
    assert_eq!(after, base + 1, "drift count must rise by exactly the one bad row");
    assert!(matches!(
        geo_readiness_check(&pool).await,
        Err(GeoReadinessError::AncestorDrift(_))
    ), "readiness must fail when ancestors drift");

    // cleanup so we don't poison other tests on the shared DB
    sqlx::query("DELETE FROM geo.subdistricts WHERE id=$1").bind(bad).execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM geo.provinces WHERE id=$1").bind(bad_prov).execute(&pool).await.unwrap();
    let _ = (p,);
}
