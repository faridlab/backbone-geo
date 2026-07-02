# backbone-geo

> **Type:** Metaphor/Backbone **domain module** (library crate, DDD 4-layer, schema-YAML SSoT).
> Owns the administrative-geography **reference master**: Country → Province → City → District →
> Subdistrict — Indonesia-first (the full Kemendagri *wilayah* set, down to kelurahan/desa, with
> postal codes and coordinates).

Every module that touches an address — party, shipping, tax, POS — resolves and validates it against
this one shared hierarchy instead of inventing its own province/city strings. Geo is **reference
data**: loaded from SQL seeds, served **read-only**, referenced across modules by **logical FK**
(store `geo.Subdistrict.id`; never a database FK or a Cargo edge).

## Quickstart

```bash
# 1. Migrate, then load the reference seed (REQUIRED deploy step — geo fails readiness until this runs)
DATABASE_URL=postgres://root:password@localhost:5432/app ./migrations/seeds/load_all.sh
```

```rust
// 2. Compose into a service, gate readiness, mount read-only routes
use backbone_geo::{GeoModule, create_guarded_geo_routes, geo_readiness_check};

let geo = GeoModule::builder().with_database(pool.clone()).build()?;
geo_readiness_check(&pool).await?;               // Err(NotSeeded)/Err(AncestorDrift(n)) otherwise

let app = axum::Router::new().merge(create_guarded_geo_routes(&geo));   // read-only
```

`create_guarded_geo_routes` is the recommended production mount (read-only). `all_crud_routes()`
exposes the full generated CRUD for trusted admin/seed tooling only; `routes()` is a `#[deprecated]`
alias. Reference data is never mutated over HTTP.

## Documentation

Start at the **[handbook index](docs/README.md)**. Key pages:

| You want… | Read |
|-----------|------|
| Why geo exists, what it refuses to do | [Philosophy](docs/philosophy.md) · [Background](docs/background.md) |
| Compose geo into a service | [Developer Guide](docs/developer-guide.md) · [Extension Guide](docs/extension-guide.md) |
| How the machine works | [Architecture](docs/architecture.md) · [Technology](docs/technology.md) |
| Extend geo safely | [Maintainer Guide](docs/maintainer-guide.md) |
| Contribute a change | [Contributing](docs/contributing.md) |
| Exact terms / entities | [Glossary](docs/glossary.md) · [PRD](docs/prd.md) · [FSD](docs/fsd.md) |
| Why these decisions | [ADR-001](docs/adr/ADR-001-geo-reference-boundary.md) · [ADR-002](docs/adr/ADR-002-seed-lifecycle-and-readiness.md) |

## Layout

```
schema/models/*.model.yaml   # SOURCE OF TRUTH — Country/Province/City/District/Subdistrict
migrations/                  # numbered create-table migrations (empty tables)
migrations/seeds/            # adopted Indonesia wilayah seed (user_owned; load_all.sh)
src/
├── lib.rs                   # GeoModule + builder + public re-exports (composition root)
├── domain/                  # entities + repository traits
├── application/             # service type aliases, DTOs, geo_readiness.rs (hand-authored)
├── infrastructure/          # repository newtypes over GenericCrudRepository
└── presentation/http/       # generated CRUD handlers + guarded_routes.rs (hand-authored, read-only)
tests/                       # geo_golden_cases.rs, integrity_probes.rs, features/geo.feature
docs/                        # the handbook
metaphor.codegen.yaml        # user_owned globs preserved across regeneration
```

Schema YAML is the single source of truth: `metaphor schema generate` regenerates everything outside
`// <<< CUSTOM … // END CUSTOM` markers and the `user_owned` files. See the
[Maintainer Guide](docs/maintainer-guide.md) before editing generated code.
