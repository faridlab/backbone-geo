# backbone-geo — Extension Guide

How a consuming service composes and references this module.

## Composing into a service

```rust
use backbone_geo::{GeoModule, create_guarded_geo_routes};

let geo = GeoModule::builder().with_database(pool.clone()).build()?;

// RECOMMENDED: read-only. Reference data is loaded via SQL seeds, never mutated over HTTP.
let app = axum::Router::new().merge(create_guarded_geo_routes(&geo));
```

Load the seed once after migrations (see `migrations/seeds/README.md`). For trusted admin/seed
tooling only, `GeoModule::all_crud_routes()` exposes generic CRUD (`routes()` is a `#[deprecated]`
alias) — do not mount it in production.

## Public / stable surface
- **Entities & DTOs** — Country/Province/City/District/Subdistrict + generated response DTOs.
- **Read routes** — `create_guarded_geo_routes`.
- **Logical FK identity** — reference `geo.Subdistrict.id` / `City.id` / `Province.id` from your own
  Address rows. **Never** add a DB foreign key across the module boundary.

## Regeneration safety
Hand-authored files (`src/presentation/http/guarded_routes.rs`, tests, `docs/**`) and the
**adopted seed data** (`migrations/seeds/**`) are `user_owned` in `metaphor.codegen.yaml` and
survive `metaphor schema schema generate --force`. The module owns the `geo` Postgres schema.
