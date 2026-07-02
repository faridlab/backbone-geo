//! Integrity probes — geo is reference data, so the guarded surface is READ-ONLY.
//! Reads work; no create/patch/delete is exposed. Hits routes via tower oneshot.
//! Requires DATABASE_URL (defaults to local dev Postgres on :5433).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use sqlx::PgPool;
use tower::ServiceExt;

use backbone_geo::{create_guarded_geo_routes, GeoModule};

async fn pool() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/backbone_geo".to_string());
    PgPool::connect(&url).await.unwrap()
}
async fn module(pool: &PgPool) -> GeoModule {
    GeoModule::builder().with_database(pool.clone()).build().unwrap()
}
async fn send(app: axum::Router, method: &str, uri: &str) -> StatusCode {
    app.oneshot(
        Request::builder().method(method).uri(uri)
            .header("content-type", "application/json").body(Body::empty()).unwrap(),
    ).await.unwrap().status()
}

// IGC-1: reads are exposed (GET list) on the guarded surface.
#[tokio::test]
async fn guarded_reads_are_exposed() {
    let pool = pool().await;
    for path in ["/countries", "/provinces", "/cities", "/districts", "/subdistricts"] {
        let status = send(create_guarded_geo_routes(&module(&pool).await), "GET", path).await;
        assert_eq!(status, StatusCode::OK, "GET {path} should be readable; got {status}");
    }
}

// IGC-2: no write verbs are exposed — geo is reference data, mutated only via seeds.
#[tokio::test]
async fn guarded_writes_are_not_exposed() {
    let pool = pool().await;
    for (method, path) in [
        ("POST", "/countries"),
        ("POST", "/provinces"),
        ("DELETE", "/subdistricts/00000000-0000-0000-0000-000000000000"),
        ("PATCH", "/cities/00000000-0000-0000-0000-000000000000"),
    ] {
        let status = send(create_guarded_geo_routes(&module(&pool).await), method, path).await;
        assert!(
            status == StatusCode::METHOD_NOT_ALLOWED || status == StatusCode::NOT_FOUND,
            "{method} {path} must not be exposed on the read-only geo surface; got {status}"
        );
    }
}
