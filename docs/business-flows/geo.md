# Business Flow — Address Resolution (Geo reference)

> Owning module: `backbone-geo` · Read-only reference data. Served by
> `src/presentation/http/guarded_routes.rs`, proven by `tests/geo_golden_cases.rs`.

Geo is not a transactional domain — it is the **administrative reference hierarchy** every other
module resolves an address against: Country → Province → City → District → Subdistrict.

## Actors
- **Consuming modules** (party, shipping, tax, POS) — resolve/validate an address chain and store
  the subdistrict/city id as a logical FK.
- **Platform admin** — loads/refreshes the seed data (out-of-band).

## Flows

### Browse the hierarchy (read)
- `GET /countries`, `/provinces`, `/cities`, `/districts`, `/subdistricts` (list + by-id), each
  filterable by ancestor id (denormalized `country_id`/`province_id`/`city_id`/`district_id`).
  Typical: a cascading address picker (province → city → district → subdistrict).

### Resolve an address to its chain
- Given a `subdistrict_id`, join up to district → city → province → country (all denormalized FKs
  are present, so no deep joins are required for filtering). Returns names + `postal_code` +
  coordinates. Proven by GGC-1.

### Load / refresh reference data (admin, out-of-band)
- Run the seed SQL (`migrations/seeds/`) in hierarchy order on a clean schema. Not an API flow —
  reference data is never mutated through HTTP (`create_guarded_geo_routes` mounts read-only).

## Invariants (DB-enforced)
- `country.isocode` unique; `subdistrict (district_id, name)` unique; denormalized ancestor ids are
  consistent with the parent chain (pre-validated in the seed, FK-checked on load).

## Not here
No geocoding/reverse-geocoding, no polygons/boundaries, no distance search — this is a flat
administrative reference set. See [golden-cases.md](golden-cases.md).
