# backbone-geo — FSD

Schema (`schema/models/*.model.yaml`) is the SSoT. Geo is read-only reference data.

## Entities

| Entity | Table | Key | Notes |
|--------|-------|-----|-------|
| Country | `geo.countries` | `isocode` unique | name, phonecode. |
| Province | `geo.provinces` | — | `country_id`. |
| City | `geo.cities` | — | `country_id`, `province_id`. |
| District | `geo.districts` | — | `country_id`, `province_id`, `city_id`. |
| Subdistrict | `geo.subdistricts` | `(district_id, name)` unique | ancestor FKs + `postal_code`, `latitude`, `longitude`. |

Tables live in the **`geo` Postgres schema**. Ancestor FKs are **denormalized** down the chain
(a subdistrict carries country/province/city/district ids) so "all subdistricts in province X"
needs no deep join. Soft-delete via `metadata` JSONB.

## Endpoints

- **Guarded (recommended)** — `create_guarded_geo_routes(&GeoModule)`: **read-only** GET (list +
  by-id) for all five entities. No create/patch/delete/upsert is mounted.
- **`GeoModule::all_crud_routes()`** — the generated full CRUD; use only for trusted admin/seed
  tooling. `routes()` is the `#[deprecated]` alias.

## Data
Loaded from `migrations/seeds/*.sql` in hierarchy order (see `migrations/seeds/README.md`). Fixed
UUIDs keep parent→child links self-consistent; FK-checked on load.

## Integration (logical FKs — no DB FK, no Cargo edge)
Consumers reference `geo.Subdistrict.id` / `City.id` / `Province.id` from their own Address rows.
Geo never imports a consuming module.

## Behavior specs
- Hooks: `schema/hooks/geo.hook.yaml` (structural invariants; no lifecycle).
- Workflows: none.
- Flows + oracle: `docs/business-flows/` + `tests/features/geo.feature`; executable oracle
  `tests/geo_golden_cases.rs` + `tests/integrity_probes.rs`.

## Non-goals
No geocoding, boundaries/polygons, distance search, or address storage. See [prd.md](prd.md) "Out".
