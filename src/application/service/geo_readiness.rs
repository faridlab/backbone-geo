//! Geo readiness guard — hand-authored (user-owned).
//!
//! Council 2026-07-02 (HIGH): the reference seed is loaded out-of-band, so a fresh tenant can boot
//! "healthy" with an EMPTY geo — and every address/tax/shipping lookup then silently returns
//! nothing. This guard makes that a hard failure: a composing service calls `geo_readiness_check`
//! at startup / in its readiness probe and refuses to serve until geo is seeded and consistent.
//!
//! It also enforces the MEDIUM finding — denormalized ancestor drift: a subdistrict/district/city
//! whose denormalized country/province/city ids disagree with its parent chain is reported, so a
//! partial re-seed or hand-correction fails readiness instead of silently returning wrong roll-ups.

use sqlx::PgPool;

use crate::infrastructure::persistence::{CountryRepository, SubdistrictRepository};

#[derive(Debug)]
pub enum GeoReadinessError {
    /// Geo holds no reference data — the seed was never loaded (see migrations/seeds/README.md).
    NotSeeded,
    /// N rows carry denormalized ancestor ids that disagree with their parent chain.
    AncestorDrift(i64),
    Db(sqlx::Error),
}

impl std::fmt::Display for GeoReadinessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoReadinessError::NotSeeded => write!(
                f,
                "geo is not seeded — load migrations/seeds/*.sql before serving (see seeds/README.md)"
            ),
            GeoReadinessError::AncestorDrift(n) => {
                write!(f, "geo has {n} rows with inconsistent denormalized ancestor ids")
            }
            GeoReadinessError::Db(e) => write!(f, "geo readiness db error: {e}"),
        }
    }
}
impl std::error::Error for GeoReadinessError {}
impl From<sqlx::Error> for GeoReadinessError {
    fn from(e: sqlx::Error) -> Self {
        GeoReadinessError::Db(e)
    }
}

/// Count rows whose denormalized ancestor ids disagree with their parent chain, across all levels.
/// Thin orchestration over `SubdistrictRepository::ancestor_drift_count` — the SQL lives in the
/// repo per the module's 4-layer rule; this free function is kept as the public API so callers
/// (and tests) don't need to know which repo hosts the CTE.
pub async fn ancestor_drift_count(pool: &PgPool) -> Result<i64, GeoReadinessError> {
    let n = SubdistrictRepository::ancestor_drift_count(pool).await?;
    Ok(n)
}

/// Startup / readiness gate. Returns `Ok(())` only if geo is seeded AND internally consistent.
/// Call this from a composing service's readiness probe so an unseeded/drifted geo can't serve.
pub async fn geo_readiness_check(pool: &PgPool) -> Result<(), GeoReadinessError> {
    let countries = CountryRepository::count_all(pool).await?;
    if countries == 0 {
        return Err(GeoReadinessError::NotSeeded);
    }
    let drift = ancestor_drift_count(pool).await?;
    if drift > 0 {
        return Err(GeoReadinessError::AncestorDrift(drift));
    }
    Ok(())
}
