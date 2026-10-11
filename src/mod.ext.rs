// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

pub use presentation::http::create_guarded_geo_routes;
pub use application::service::{
    ancestor_drift_count, geo_readiness_check, GeocodeError, GeocodeProvider, GeocodeQuery,
    GeocodeService, GeoReadinessError,
};
