<!-- Reader: App developer · Mode: Tutorial → How-to -->
# Developer Guide

Get from zero to a service that serves the geo hierarchy — and refuses to serve when geo is empty.
For the deeper stable-surface contract, see the [Extension Guide](extension-guide.md).

> **Example provenance (2026-07-02).** The snippets below are drawn verbatim from geo's own source
> and doc comments (`src/lib.rs`, `guarded_routes.rs`, `geo_readiness.rs`, `migrations/seeds/`), so
> they match the real API. They were **not** independently compiled in this doc session (the
> framework crates fetch from git and need a live PostgreSQL); treat them as accurate to the source,
> and let `metaphor dev test` be the run-time proof.

## Install

Add geo as a path (or git) dependency in your service's `Cargo.toml`:

```toml
[dependencies]
backbone-geo = { path = "../backbone-geo" }   # or: git = "…", tag = "vX.Y.Z"
```

You also need a reachable **PostgreSQL** and the **`psql`** client (for the seed loader).

## Quickstart — the smallest thing that runs

Three steps: migrate, **seed** (required), mount read-only routes behind the readiness gate.

**1. Create the schema and load the reference data.** Run migrations, then the seed loader — the
seed is a required deploy step, not optional:

```bash
DATABASE_URL=postgres://root:password@localhost:5432/app ./migrations/seeds/load_all.sh
# >> seeding country … province … city … district … subdistrict
# >> geo seed complete
```

**2. Compose geo into your service and gate readiness on it:**

```rust
use backbone_geo::{GeoModule, create_guarded_geo_routes, geo_readiness_check};

let geo = GeoModule::builder()
    .with_database(pool.clone())
    .build()?;

// Contract: refuse to serve on an unseeded or drifted geo.
geo_readiness_check(&pool).await?;   // Err(NotSeeded) / Err(AncestorDrift(n)) otherwise

// RECOMMENDED mount: read-only. Reference data is seeded via SQL, never written over HTTP.
let app = axum::Router::new().merge(create_guarded_geo_routes(&geo));
```

**3. Verify it serves:**

```bash
curl -s localhost:8080/provinces | jq 'length'   # 38 (Indonesia seed)
curl -s -o /dev/null -w '%{http_code}\n' -X POST localhost:8080/provinces   # 405 — no HTTP writes
```

Expected: 38 provinces readable; any write returns `405/404`.

## Key concepts

- **Read-only reference data** — geo is loaded from seeds and served read-only; there is no runtime
  write path. See [Philosophy](philosophy.md).
- **Logical FK** — store `geo.Subdistrict.id` / `City.id` / `Province.id` on your own rows. **Never**
  add a database foreign key across the boundary. See [Extension Guide](extension-guide.md).
- **Denormalized ancestors** — every row carries its full ancestor chain, so filtering by any
  ancestor id is one indexed equality, no joins. See [Architecture](architecture.md).
- **Readiness gate** — `geo_readiness_check(&pool)` makes an empty/drifted geo a hard failure. Wire
  it into your startup and readiness probe. See [ADR-002](adr/ADR-002-seed-lifecycle-and-readiness.md).

## Recipes

### How do I resolve an address to its full chain?
`GET /subdistricts/:id` returns the subdistrict plus its denormalized `district_id`, `city_id`,
`province_id`, `country_id`, `postal_code`, and coordinates — no follow-up joins needed. To show
names up the chain, read each ancestor by id (or issue filtered list calls). Pinned by golden case
**GGC-1**.

### How do I build a cascading address picker?
Filter each level by its parent id (denormalized, so it's a single indexed lookup):
```
GET /provinces?country_id=<ID>
GET /cities?province_id=<province>
GET /districts?city_id=<city>
GET /subdistricts?district_id=<district>
```

### How do I store a resolved address on my own entity?
Keep a `subdistrict_id` (and/or `city_id`) column on your row as a **plain UUID** — a logical FK.
Do not declare a DB `REFERENCES geo.subdistricts(...)` constraint; the two schemas migrate
independently.

### How do I make my service refuse to boot on empty geo?
Call `geo_readiness_check(&pool).await?` in startup and in your `/readyz` probe. In CI/prod set
`GEO_REQUIRE_SEED=1` so geo's smoke test fails (not skips) on an unseeded DB.

### How do I refresh the reference data?
Re-run `load_all.sh` against a **clean** `geo` schema — load the whole set, never partially, or you
risk ancestor drift. See [Maintainer Guide § refresh the seed](maintainer-guide.md#task-refresh-or-fix-the-seed).

## Configuration

Geo ships `config/application.yml` (+ `-dev` / `-prod` overrides) as sensible defaults; a composing
service usually supplies its own `database.url` and server settings. The options that matter:

| Option | Default | When to change |
|--------|---------|----------------|
| `database.url` | `postgresql://root:password@localhost:5432/skeletondb` | Always — point at your DB. |
| `database.max_connections` | `10` | Under heavy read concurrency. |
| `entities.<e>.pagination.default_limit` / `max_limit` | `20` / `100` | Larger address-picker pages. |
| `entities.<e>.cache_ttl` | `300` | If you front reads with a cache (geo ships none by default). |
| `GEO_REQUIRE_SEED` (env) | unset (local: skip) | Set `=1` in CI/prod so an empty geo fails tests. |

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| `geo_readiness_check` returns `NotSeeded` | Migrations ran but the seed didn't | Run `./migrations/seeds/load_all.sh` with `DATABASE_URL` set. |
| `geo_readiness_check` returns `AncestorDrift(n)` | A partial/hand re-seed left denormalized ancestor ids inconsistent | Reload the **whole** seed into a clean schema; never partial. |
| Address dropdowns are blank, no error | Empty geo + logical FKs raise no referential error — just empty JOINs | This is exactly what the readiness gate prevents; wire it in. |
| `POST /provinces` returns 405/404 | You mounted the guarded (read-only) router — correct behavior | Reference data has no HTTP write path; correct via re-seed. |
| Writes "work" unexpectedly | You mounted `all_crud_routes()` / `routes()` | Use `create_guarded_geo_routes` in production. |
| Seed loader errors on FK | Loaded out of hierarchy order | Use `load_all.sh` (country→province→city→district→subdistrict). |
