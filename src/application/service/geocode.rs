//! The typed geocode port (hand-authored; user-owned; see
//! `metaphor.codegen.yaml`).
//!
//! Port of the upstream `base_geolocalize` surface with its two defects
//! fixed structurally:
//!
//! 1. **Fail-closed, never sentinel coordinates.** Upstream maps every
//!    provider error to `None` and leaves the record's coordinates at
//!    (0.0, 0.0) — an unset value indistinguishable from a real fix.
//!    This port has no "unset" coordinate state at all: a geocode call
//!    either returns validated [`Coordinates`] or one of the typed
//!    [`GeocodeError`] refusals. (0.0, 0.0) can still be *returned by a
//!    real provider* for a genuine Gulf-of-Guinea fix, but it can never
//!    mean "did not answer".
//! 2. **Rate-limited.** Upstream has no client-side limit at all — a
//!    batch geocode against a ToS-limited public endpoint (Nominatim's
//!    1 req/s) can get the deployment's IP banned. Every call passes
//!    the per-caller fixed-window [`RateFence`] BEFORE the provider is
//!    consulted.
//!
//! Provider selection follows the family's fail-closed selector
//! posture (the website captcha knob's shape): unset refuses typed;
//! `"stub"` arms the deterministic offline provider (dev plumbing
//! only); any other selector arms the [`UnknownGeocodeProvider`] arm
//! that refuses every call naming the selector — never a silent
//! fallback to another provider.
//!
//! The port is a LIBRARY surface: no HTTP composer ships here. The
//! future consumer (logistics / point-of-sale maps) mounts it with its
//! own caller-IP resolution — the same reason `geo` ships no
//! trusted-proxy helper of its own.
//!
//! Network providers (Nominatim, Google) are deliberately NOT in this
//! increment: they arrive with their consumer, behind credentials from
//! the sapiens credential store or a dedicated env knob declared in
//! both deployment env templates in the same change that wires them.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

/// The provider selector env knob. Unset or empty = the refusing
/// default (the port exists but answers no coordinates); `stub` = the
/// deterministic offline provider; anything else arms the fail-closed
/// unknown-selector refusal.
pub const GEOCODE_PROVIDER_ENV: &str = "GEOCODE_PROVIDER";

/// The per-caller fence default: 30 calls per rolling 60-second window.
/// Generous for interactive use, far below the batch shapes that trip
/// public-endpoint bans upstream.
pub const DEFAULT_GEOCODE_MAX_PER_WINDOW: u64 = 30;
/// The fence window length in seconds.
pub const DEFAULT_GEOCODE_WINDOW_SECS: u64 = 60;

/// A free-text address component set (the upstream
/// `geo_query_address` shape, typed and bounded). At least one
/// component must be non-empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GeocodeQuery {
    pub street: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

impl GeocodeQuery {
    /// Bound and sanity-check the payload. Every component is trimmed
    /// and capped at 200 characters (over-length is the typed 422,
    /// never a silently unfiltered passthrough); at least one
    /// non-empty component must remain.
    pub fn validate(&self) -> Result<(), GeocodeError> {
        const MAX_COMPONENT: usize = 200;
        let bounded = |v: &Option<String>| -> Result<Option<String>, GeocodeError> {
            match v.as_deref().map(str::trim) {
                None | Some("") => Ok(None),
                Some(s) if s.len() > MAX_COMPONENT => Err(GeocodeError::InvalidQuery(format!(
                    "address components are capped at {MAX_COMPONENT} characters"
                ))),
                Some(s) => Ok(Some(s.to_string())),
            }
        };
        let cleaned = GeocodeQuery {
            street: bounded(&self.street)?,
            zip: bounded(&self.zip)?,
            city: bounded(&self.city)?,
            country: bounded(&self.country)?,
        };
        if cleaned == GeocodeQuery::default() {
            return Err(GeocodeError::InvalidQuery(
                "at least one address component (street, zip, city, country) is required"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

/// A validated coordinate fix. Construction refuses non-finite and
/// out-of-range values, so no provider can smuggle a garbage or
/// sentinel-latitude answer through the typed surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    /// Validate and build. Latitude must lie in [-90, 90], longitude in
    /// [-180, 180], both finite — anything else is the typed
    /// provider-side refusal, never a clamped or defaulted value.
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, GeocodeError> {
        if !latitude.is_finite()
            || !longitude.is_finite()
            || !(-90.0..=90.0).contains(&latitude)
            || !(-180.0..=180.0).contains(&longitude)
        {
            return Err(GeocodeError::ProviderUnavailable(
                "provider answered a coordinate outside the valid range".to_string(),
            ));
        }
        Ok(Self { latitude, longitude })
    }
}

/// The typed refusal surface. Every failure mode of a geocode call is
/// one of these arms — there is no `None`-means-unknown path.
#[derive(Debug, thiserror::Error)]
pub enum GeocodeError {
    /// No provider is configured (the knob is unset / `none`). The
    /// port answers this instead of coordinates — never (0.0, 0.0).
    #[error("no geocode provider is configured")]
    ProviderNotConfigured,
    /// The selector names no provider this deployment knows — armed
    /// fail-closed rather than silently falling back.
    #[error("the geocode provider selector names no known provider: {0}")]
    ProviderUnknown(String),
    /// The query payload failed validation (empty or over-length).
    #[error("invalid geocode query: {0}")]
    InvalidQuery(String),
    /// The provider answered with no fix for the query — a typed
    /// absence, distinct from every error arm.
    #[error("the provider found no match for this address")]
    NoMatch,
    /// The per-caller rate fence refused the call.
    #[error("geocode rate limited; retry after {retry_after_seconds}s")]
    RateLimited { retry_after_seconds: u64 },
    /// The provider is configured but cannot serve right now
    /// (unreachable, refused its credentials, or answered garbage).
    #[error("geocode provider unavailable: {0}")]
    ProviderUnavailable(String),
}

impl GeocodeError {
    /// The HTTP status a mounting consumer maps this refusal to.
    pub fn http_status(&self) -> u16 {
        match self {
            GeocodeError::ProviderNotConfigured => 503,
            GeocodeError::ProviderUnknown(_) => 503,
            GeocodeError::InvalidQuery(_) => 422,
            GeocodeError::NoMatch => 404,
            GeocodeError::RateLimited { .. } => 429,
            GeocodeError::ProviderUnavailable(_) => 502,
        }
    }

    /// The stable machine code a mounting consumer emits.
    pub fn code(&self) -> &'static str {
        match self {
            GeocodeError::ProviderNotConfigured => "geocode_provider_not_configured",
            GeocodeError::ProviderUnknown(_) => "geocode_provider_unknown",
            GeocodeError::InvalidQuery(_) => "geocode_invalid_query",
            GeocodeError::NoMatch => "geocode_no_match",
            GeocodeError::RateLimited { .. } => "geocode_rate_limited",
            GeocodeError::ProviderUnavailable(_) => "geocode_provider_unavailable",
        }
    }
}

/// A geocoding provider. Implementations answer a validated fix or a
/// typed refusal; they never debounce, cache, or rate-limit (the
/// [`GeocodeService`] owns the fence) and never return sentinel
/// coordinates for "no answer" — that is [`GeocodeError::NoMatch`].
#[async_trait]
pub trait GeocodeProvider: Send + Sync {
    /// The stable provider name (labels, logs, error context).
    fn name(&self) -> &'static str;
    /// Resolve one address query.
    async fn geocode(&self, query: &GeocodeQuery) -> Result<Coordinates, GeocodeError>;
}

/// The refusing default: the port exists, no provider is configured.
/// Every call answers [`GeocodeError::ProviderNotConfigured`] — the
/// fail-closed replacement for upstream's unset-coordinates behavior.
#[derive(Debug, Default, Clone, Copy)]
pub struct RefusingGeocodeProvider;

#[async_trait]
impl GeocodeProvider for RefusingGeocodeProvider {
    fn name(&self) -> &'static str {
        "none"
    }
    async fn geocode(&self, _query: &GeocodeQuery) -> Result<Coordinates, GeocodeError> {
        Err(GeocodeError::ProviderNotConfigured)
    }
}

/// The deterministic offline provider (dev plumbing only). Answers the
/// same fixed fix for every valid query — enough to exercise the full
/// typed pipeline and a consumer's wiring without network or
/// credentials. It is NOT a geocoder: no dev environment should read
/// its output as a real location.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubGeocodeProvider;

/// The stub's fixed answer (central Jakarta — a recognizably "wrong on
/// purpose" value for any non-Jakarta query).
impl StubGeocodeProvider {
    /// The stub's latitude component.
    pub const STUB_LATITUDE: f64 = -6.200_000;
    /// The stub's longitude component.
    pub const STUB_LONGITUDE: f64 = 106.816_666;
}

#[async_trait]
impl GeocodeProvider for StubGeocodeProvider {
    fn name(&self) -> &'static str {
        "stub"
    }
    async fn geocode(&self, query: &GeocodeQuery) -> Result<Coordinates, GeocodeError> {
        query.validate()?;
        Coordinates::new(Self::STUB_LATITUDE, Self::STUB_LONGITUDE)
    }
}

/// The fail-closed unknown-selector arm: the knob named a provider this
/// deployment does not know, so every call refuses naming it — the
/// operator gets a loud, specific error instead of a silent fallback.
#[derive(Debug, Clone)]
pub struct UnknownGeocodeProvider {
    /// The unrecognized selector, kept for the refusal message.
    pub selector: String,
}

#[async_trait]
impl GeocodeProvider for UnknownGeocodeProvider {
    fn name(&self) -> &'static str {
        "unknown"
    }
    async fn geocode(&self, _query: &GeocodeQuery) -> Result<Coordinates, GeocodeError> {
        Err(GeocodeError::ProviderUnknown(self.selector.clone()))
    }
}

/// A per-caller fixed-window rate fence (the module family's
/// `FixedWindows` shape: in-memory, key-scoped, poison-recovering).
/// Shared-nothing deployments get one fence per process; that is the
/// same posture the family's intake throttles ship.
#[derive(Debug)]
pub struct RateFence {
    inner: Mutex<HashMap<String, (u64, u64)>>, // key -> (window_start_unix, count)
    max_per_window: u64,
    window_secs: u64,
}

impl RateFence {
    /// Build a fence allowing `max_per_window` calls per
    /// `window_secs` per key.
    pub fn new(max_per_window: u64, window_secs: u64) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            max_per_window: max_per_window.max(1),
            window_secs: window_secs.max(1),
        }
    }

    /// Record a hit for `key` at wall-clock now. `None` = allowed;
    /// `Some(retry_after_seconds)` = refused.
    pub fn allow(&self, key: &str) -> Option<u64> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.allow_at(key, now)
    }

    /// The fence decision at an explicit unix-seconds clock — the
    /// deterministic seam the probes drive (no sleeping).
    pub fn allow_at(&self, key: &str, now_unix: u64) -> Option<u64> {
        let mut guard = match self.inner.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        let (start, count) = match guard.get_mut(key) {
            Some((start, count)) if now_unix.saturating_sub(*start) < self.window_secs => {
                (*start, *count)
            }
            _ => {
                guard.insert(key.to_string(), (now_unix, 0));
                (now_unix, 0)
            }
        };
        if count >= self.max_per_window {
            let retry_after = self.window_secs - now_unix.saturating_sub(start);
            return Some(retry_after.max(1));
        }
        guard.insert(key.to_string(), (start, count + 1));
        None
    }
}

/// The geocode service: the one entry point consumers call. Order is
/// fixed — validate, fence, provider — so an invalid query never burns
/// fence budget and a fenced caller never reaches the provider.
#[derive(Clone)]
pub struct GeocodeService {
    provider: Arc<dyn GeocodeProvider>,
    fence: Arc<RateFence>,
}

impl GeocodeService {
    /// Build with an explicit provider and fence (tests and hosts with
    /// their own limits).
    pub fn new(provider: Arc<dyn GeocodeProvider>, fence: RateFence) -> Self {
        Self {
            provider,
            fence: Arc::new(fence),
        }
    }

    /// Resolve the provider from [`GEOCODE_PROVIDER_ENV`] with the
    /// family's fail-closed selector posture and the default fence.
    pub fn from_env() -> Self {
        let raw = std::env::var(GEOCODE_PROVIDER_ENV).unwrap_or_default();
        let provider: Arc<dyn GeocodeProvider> = match raw.trim().to_ascii_lowercase().as_str() {
            "" | "none" => Arc::new(RefusingGeocodeProvider),
            "stub" => Arc::new(StubGeocodeProvider),
            other => {
                tracing::error!(
                    provider = %other,
                    env = GEOCODE_PROVIDER_ENV,
                    "unknown geocode provider selector — the port stays armed on its \
                     fail-closed refusal (never a silent fallback)"
                );
                Arc::new(UnknownGeocodeProvider {
                    selector: other.to_string(),
                })
            }
        };
        Self::new(
            provider,
            RateFence::new(
                DEFAULT_GEOCODE_MAX_PER_WINDOW,
                DEFAULT_GEOCODE_WINDOW_SECS,
            ),
        )
    }

    /// The mounted provider's name (labels, logs).
    pub fn provider_name(&self) -> &'static str {
        self.provider.name()
    }

    /// Geocode one query for one caller. `caller_key` is the
    /// caller-identity the fence scopes to (the consumer's resolved
    /// caller IP).
    pub async fn geocode(
        &self,
        caller_key: &str,
        query: &GeocodeQuery,
    ) -> Result<Coordinates, GeocodeError> {
        query.validate()?;
        if let Some(retry_after_seconds) = self.fence.allow(caller_key) {
            return Err(GeocodeError::RateLimited { retry_after_seconds });
        }
        self.provider.geocode(query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query() -> GeocodeQuery {
        GeocodeQuery {
            street: Some("Jalan Jenderal Sudirman 1".into()),
            city: Some("Jakarta".into()),
            zip: None,
            country: Some("Indonesia".into()),
        }
    }

    #[tokio::test]
    async fn refusing_default_never_answers_coordinates() {
        let svc = GeocodeService::new(
            Arc::new(RefusingGeocodeProvider),
            RateFence::new(30, 60),
        );
        let err = svc.geocode("203.0.113.9", &query()).await.unwrap_err();
        assert!(matches!(err, GeocodeError::ProviderNotConfigured));
        assert_eq!(err.http_status(), 503);
        assert_eq!(err.code(), "geocode_provider_not_configured");
    }

    #[tokio::test]
    async fn stub_provider_answers_the_fixed_fix() {
        let svc = GeocodeService::new(Arc::new(StubGeocodeProvider), RateFence::new(30, 60));
        let fix = svc.geocode("203.0.113.9", &query()).await.unwrap();
        assert_eq!(fix.latitude, StubGeocodeProvider::STUB_LATITUDE);
        assert_eq!(fix.longitude, StubGeocodeProvider::STUB_LONGITUDE);
    }

    #[tokio::test]
    async fn empty_query_is_the_typed_422_and_burns_no_fence_budget() {
        let svc = GeocodeService::new(Arc::new(StubGeocodeProvider), RateFence::new(1, 60));
        let err = svc
            .geocode("203.0.113.9", &GeocodeQuery::default())
            .await
            .unwrap_err();
        assert!(matches!(err, GeocodeError::InvalidQuery(_)));
        // The valid call still fits the size-1 fence — validation ran first.
        assert!(svc.geocode("203.0.113.9", &query()).await.is_ok());
    }

    #[tokio::test]
    async fn fence_refuses_before_the_provider_is_consulted() {
        let svc = GeocodeService::new(Arc::new(StubGeocodeProvider), RateFence::new(1, 60));
        assert!(svc.geocode("198.51.100.7", &query()).await.is_ok());
        let err = svc.geocode("198.51.100.7", &query()).await.unwrap_err();
        assert!(matches!(
            err,
            GeocodeError::RateLimited { retry_after_seconds: 1..=60 }
        ));
        assert_eq!(err.http_status(), 429);
        // A different caller is a different bucket.
        assert!(svc.geocode("198.51.100.8", &query()).await.is_ok());
    }

    #[tokio::test]
    async fn fence_window_rolls_over_deterministically() {
        let fence = RateFence::new(1, 60);
        assert_eq!(fence.allow_at("k", 1_000), None);
        assert_eq!(fence.allow_at("k", 1_030), Some(30));
        // Past the window the bucket resets.
        assert_eq!(fence.allow_at("k", 1_061), None);
    }

    #[tokio::test]
    async fn unknown_selector_refuses_naming_the_selector() {
        let svc = GeocodeService::new(
            Arc::new(UnknownGeocodeProvider { selector: "nominatim".into() }),
            RateFence::new(30, 60),
        );
        let err = svc.geocode("203.0.113.9", &query()).await.unwrap_err();
        assert!(matches!(err, GeocodeError::ProviderUnknown(s) if s == "nominatim"));
    }

    #[tokio::test]
    async fn coordinates_constructor_refuses_out_of_range_answers() {
        assert!(Coordinates::new(91.0, 0.0).is_err());
        assert!(Coordinates::new(0.0, -181.0).is_err());
        assert!(Coordinates::new(f64::NAN, 0.0).is_err());
        // The Gulf of Guinea fix itself stays a legal value — it can
        // arrive from a provider, it just cannot MEAN "unset".
        assert!(Coordinates::new(0.0, 0.0).is_ok());
    }
}
