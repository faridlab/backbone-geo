//! Guarded route composition — the RECOMMENDED way to mount the geo module.
//!
//! Hand-authored (user-owned; see `metaphor.codegen.yaml`). Geo is **reference master data**: the
//! administrative hierarchy is loaded from public datasets via SQL seeds (`migrations/seeds/`), not
//! authored through the API. So every entity is mounted **READ-ONLY** here — there is no legitimate
//! runtime write path, and the generated generic CRUD (create/patch/delete/upsert) is not exposed.
//! Reloading/corrections happen out-of-band via a seed migration.

use axum::Router;

use crate::GeoModule;

use super::{
    create_city_read_routes, create_country_read_routes, create_district_read_routes,
    create_province_read_routes, create_subdistrict_read_routes,
};

/// Mount the geo module read-only. **Prefer this over `GeoModule::all_crud_routes()` for any real
/// deployment** — reference data must not be mutated through the API.
pub fn create_guarded_geo_routes(m: &GeoModule) -> Router {
    Router::new()
        .merge(create_country_read_routes(m.country_service.clone()))
        .merge(create_province_read_routes(m.province_service.clone()))
        .merge(create_city_read_routes(m.city_service.clone()))
        .merge(create_district_read_routes(m.district_service.clone()))
        .merge(create_subdistrict_read_routes(m.subdistrict_service.clone()))
}
