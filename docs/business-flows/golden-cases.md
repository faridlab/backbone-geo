# Geo — Golden Cases (the oracle)

Exact expected results, mirroring `tests/geo_golden_cases.rs` and `tests/integrity_probes.rs`.

## Hierarchy (`tests/geo_golden_cases.rs`)

| Case | Input | Expected |
|------|-------|----------|
| **GGC-1** | insert a country→province→city→district→subdistrict chain; resolve by subdistrict id | full chain names resolve (country/province/city/district/subdistrict) + `postal_code`. |
| **GGC-2** | insert a second subdistrict with the same name in the same district | rejected (unique `(district_id, name)`). |
| **GGC-3** | subdistrict with latitude/longitude | coordinates round-trip. |
| **GGC-4** (smoke) | the loaded Indonesia seed | 38 provinces under isocode `ID`; DKI Jakarta > 200 subdistricts. **Fails** (not skips) if empty when `GEO_REQUIRE_SEED=1`. |
| **GGC-5** | `geo_readiness_check` on the seeded DB | `Ok`; `ancestor_drift_count == 0` (seed is internally consistent). |
| **GGC-6** | a crafted subdistrict whose `province_id` ≠ its district's | drift count rises by 1; readiness returns `AncestorDrift`. |

## Read-only surface (`tests/integrity_probes.rs`)

| Case | Input via guarded routes | Expected |
|------|--------------------------|----------|
| **IGC-1** | `GET /countries|/provinces|/cities|/districts|/subdistricts` | `200` (readable). |
| **IGC-2** | `POST /countries`, `POST /provinces`, `DELETE /subdistricts/:id`, `PATCH /cities/:id` | `405/404` — geo is reference data; no HTTP writes. |

## Loaded seed (reference)
1 country (ID), 38 provinces, 489 cities, 7,274 districts, 83,741 subdistricts; 0 orphans.
