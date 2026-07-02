<!-- Reader: All · Mode: Reference -->
# Glossary — Ubiquitous Language

One term, one meaning, used the same way across the whole handbook, the schema, and the code. When
a doc and this list disagree, fix the doc.

## The hierarchy

| Term | Definition |
|------|------------|
| **Administrative hierarchy** | The nested set of government regions geo owns: Country → Province → City → District → Subdistrict. |
| **Country** | Top level. Table `geo.countries`. Key field `isocode` (unique); also `name`, `phonecode`. Indonesia (`ID`) is the seeded country. |
| **Province** | Second level (*provinsi*). Table `geo.provinces`. Carries `country_id`. 38 rows seeded. |
| **City** | Third level (*kota/kabupaten*). Table `geo.cities`. Carries `country_id`, `province_id`. 489 rows seeded. |
| **District** | Fourth level (*kecamatan*). Table `geo.districts`. Carries `country_id`, `province_id`, `city_id`. 7,274 rows seeded. |
| **Subdistrict** | Leaf level (*kelurahan/desa*). Table `geo.subdistricts`. Carries the full ancestor chain + `postal_code`, `latitude`, `longitude`. Unique on `(district_id, name)`. 83,741 rows seeded. This is the level a party/shipping/tax address resolves down to. |
| **Wilayah** | Indonesian for "region/territory"; the umbrella term for the Kemendagri administrative dataset geo seeds. |

## Concepts

| Term | Definition |
|------|------------|
| **Reference (master) data** | Data that is looked up, not transacted — loaded from datasets, changed in bulk offline, served read-only. Geo is reference data; `backbone-party`'s addresses are transactional data that *reference* it. |
| **Denormalized ancestor FK** | An ancestor id stored directly on a descendant row (e.g. `subdistrict.province_id`) so filtering by any ancestor is a single indexed equality, not a join. Distinct from a *logical FK*, which crosses a module boundary. |
| **Logical FK** | A cross-module reference stored as a plain value (e.g. a consumer's `subdistrict_id`), with **no** database foreign-key constraint and **no** Cargo dependency edge. The only way another module links into geo. |
| **`geo` schema** | The dedicated PostgreSQL schema geo owns. Every geo table is `geo.<table>`. No other module writes into it. |
| **Ancestor drift** | The fault where a row's denormalized ancestor id disagrees with its parent chain (e.g. a subdistrict's `province_id` ≠ its district's `province_id`). Counted by `ancestor_drift_count`; reported as `AncestorDrift(n)` by the readiness check. |
| **Readiness gate / guard** | `geo_readiness_check(&pool)` — returns `Ok` only if geo is seeded **and** internally consistent; `NotSeeded` on empty, `AncestorDrift(n)` on drift. A composing service must gate startup/readiness on it. |
| **Seed / seeding** | Loading the reference dataset from `migrations/seeds/*.sql` (via `load_all.sh`) in hierarchy order. A **required deploy step**, not optional. |

## Framework / codegen terms

| Term | Definition |
|------|------------|
| **Schema YAML (SSoT)** | `schema/models/*.model.yaml` — the single source of truth. Entities, DTOs, repositories, services, handlers, and migrations are generated from it. |
| **`// <<< CUSTOM … // END CUSTOM`** | A regeneration-safe marker. Content between the markers is preserved when a generated file is re-generated. |
| **`user_owned`** | A glob in `metaphor.codegen.yaml` naming whole files the generator never reads, merges, or deletes (geo's `guarded_routes.rs`, `geo_readiness.rs`, tests, seeds, docs). |
| **`GenericCrudService` / `GenericCrudRepository` / `BackboneCrudHandler`** | Framework-supplied generics. Geo's services are **type aliases** over the service, its repositories **thin newtypes** over the repository; the handler generates the 12 standard CRUD endpoints. |
| **Guarded routes** | `create_guarded_geo_routes(&GeoModule)` — the **read-only** production mount. |
| **`all_crud_routes()` / `routes()`** | The full unvalidated 12-endpoint CRUD mount, for trusted admin/seed tooling only. `routes()` is a `#[deprecated]` alias. Not for production. |
| **GGC / IGC** | Golden-case ids: **GGC-n** (hierarchy & readiness, `tests/geo_golden_cases.rs`); **IGC-n** (read-only surface, `tests/integrity_probes.rs`). See [golden-cases.md](business-flows/golden-cases.md). |
| **`GEO_REQUIRE_SEED`** | Env flag; when `=1` (CI/prod) the seed smoke test **fails** instead of skipping on an empty DB. |

## The twelve CRUD endpoints (generated per entity)

`list`, `create`, `get`, `update`, `patch`, `soft_delete`, `restore`, `empty_trash`,
`bulk_create`, `upsert`, `find_by_id`, `list_deleted`. In geo, only the **read** subset (`list`,
`get`/`find_by_id`) is mounted in production via the guarded routes.
