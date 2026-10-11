// This directory's extension (hand-written; ADR-0031). The generated mod.rs (lib.rs for
// src/) pulls it in with include!, so its lines resolve against this directory.

pub mod geocode;
pub mod geo_readiness;
pub use geocode::{
    Coordinates, GeocodeError, GeocodeProvider, GeocodeQuery, GeocodeService, RateFence,
    RefusingGeocodeProvider, StubGeocodeProvider, UnknownGeocodeProvider,
    DEFAULT_GEOCODE_MAX_PER_WINDOW, DEFAULT_GEOCODE_WINDOW_SECS, GEOCODE_PROVIDER_ENV,
};
pub use geo_readiness::{ancestor_drift_count, geo_readiness_check, GeoReadinessError};
