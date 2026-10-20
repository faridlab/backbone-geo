//! Public-route probes — the schema declares every geo entity's reads `public: [read]`, so those
//! routes answer an anonymous caller, while every other gate of the same entities stays behind
//! the permission layer: nothing is public but the declared reads. Hits the module's
//! `public_routes()` in-process via tower oneshot — the router a host mounts outside its
//! authentication layer.
//! Requires DATABASE_URL (defaults to local dev Postgres on :5433).

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use backbone_auth::middleware::AuthContext;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

use backbone_geo::GeoModule;

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/backbone_geo".to_string());
    PgPool::connect(&url).await.unwrap()
}

async fn module(pool: &PgPool) -> GeoModule {
    GeoModule::builder().with_database(pool.clone()).build().unwrap()
}

/// One request through `public_routes()`, optionally carrying the `AuthContext` a host's
/// authentication layer would have attached (`None` for an anonymous caller).
async fn call(
    m: &GeoModule,
    method: &str,
    uri: &str,
    body: &str,
    caller: Option<AuthContext>,
) -> (StatusCode, String) {
    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    if let Some(ctx) = caller {
        req.extensions_mut().insert(ctx);
    }
    let res = m.public_routes().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = to_bytes(res.into_body(), 1 << 20).await.unwrap();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

/// A caller the authentication layer admits, holding exactly `perms`.
fn caller_holding(perms: &[&str]) -> AuthContext {
    let mut ctx = AuthContext::new("public-route-probe".into());
    ctx.permissions = perms.iter().map(|s| s.to_string()).collect();
    ctx
}

/// A unique isocode inside the column's three characters (the schema's `@max(3)` is a database
/// check, so the fixture's uniqueness must fit the same three characters the rule allows).
fn uq_isocode() -> String {
    const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let bits = u128::from_be_bytes(*Uuid::new_v4().as_bytes());
    (0..3).map(|k| CHARS[usize::try_from((bits >> (8 * k)) % 36).unwrap()] as char).collect()
}

// PRP-1: the declared public reads — list on every entity, and get-one — answer a caller with no
// authentication at all.
#[tokio::test]
async fn public_reads_answer_an_anonymous_caller() {
    let pool = pool().await;
    let m = module(&pool).await;
    for path in ["/countries", "/provinces", "/cities", "/districts", "/subdistricts"] {
        let (status, body) = call(&m, "GET", path, "", None).await;
        assert_eq!(status, StatusCode::OK, "anonymous GET {path}: {body}");
    }

    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO geo.countries (id, isocode, name, phonecode) VALUES ($1,$2,'Public probe',62)")
        .bind(id)
        .bind(uq_isocode())
        .execute(&pool)
        .await
        .unwrap();
    let (status, body) = call(&m, "GET", &format!("/countries/{id}"), "", None).await;
    assert_eq!(status, StatusCode::OK, "anonymous GET /countries/{id}: {body}");
    sqlx::query("DELETE FROM geo.countries WHERE id=$1").bind(id).execute(&pool).await.unwrap();
}

// PRP-2: a gate that is not a declared public read refuses an anonymous caller with 401 — geo has
// no non-public entity (the whole hierarchy is one public reference family), so the fail-closed
// default is proven on the write gates of the same public models.
#[tokio::test]
async fn guarded_gates_refuse_an_anonymous_caller() {
    let pool = pool().await;
    let m = module(&pool).await;
    for (method, path) in [
        ("POST", "/countries"),
        ("PATCH", "/countries/00000000-0000-0000-0000-000000000000"),
        ("DELETE", "/subdistricts/00000000-0000-0000-0000-000000000000"),
    ] {
        let (status, body) = call(&m, method, path, "{}", None).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "{method} {path} must refuse an anonymous caller: {body}"
        );
        assert!(body.contains("authentication_required"), "{method} {path}: {body}");
    }
}

// PRP-3: a write to a public model still requires its permission — an authenticated caller
// without it is refused with the violation naming it, and a holder passes the gate.
#[tokio::test]
async fn a_write_to_a_public_model_still_requires_its_permission() {
    let pool = pool().await;
    let m = module(&pool).await;

    let (status, body) = call(&m, "POST", "/countries", "{}", Some(caller_holding(&[]))).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
    assert!(
        body.contains("permission_required") && body.contains("country:create"),
        "the refusal must name the permission: {body}"
    );

    // Holding it, the gate passes and the write reaches the field guard, which refuses the blank
    // required fields — and creates no row.
    let before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM geo.countries")
        .fetch_one(&pool)
        .await
        .unwrap();
    let (status, body) = call(
        &m,
        "POST",
        "/countries",
        r#"{"isocode":"","name":"","phonecode":0}"#,
        Some(caller_holding(&["country:create"])),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "the permission gate passed and the field guard refused: {body}"
    );
    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM geo.countries")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(before, after, "a refused write must not create a row");
}
