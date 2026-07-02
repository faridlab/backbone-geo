<!-- Reader: Evaluator + Maintainer · Mode: Explanation -->
# Technology & the "Why"

The stack, and the reasoning behind each choice. Versions are from
[`Cargo.toml`](../Cargo.toml) as of 2026-07-02; the framework crates track the
`backbone-framework` repo's `main` branch.

## The choices at a glance

| Concern | Choice | Why this, in one line | Rejected alternative |
|---------|--------|-----------------------|----------------------|
| Language / edition | **Rust 2021**, `[lib]` only | Workspace standard; a module is a library, never a binary | A binary target (wrong project type for a `module`) |
| Web transport | **Axum 0.7** | Tower-native, `Router` composes cleanly for read-only mounts | Actix, warp — not the workspace convention |
| Database | **SQLx 0.8 / PostgreSQL** | Compile-time-checked queries; a dedicated `geo` schema | Diesel (heavier macro layer); an ORM that hides SQL |
| Async runtime | **Tokio 1.x** (`full`) | The de-facto async runtime the whole stack shares | async-std |
| Numbers | **rust_decimal 1.36** | Exact `latitude`/`longitude` (`decimal(12,8)`), no float drift | `f64` coordinates (rounding error) |
| Domain errors | **thiserror 1.0** | Typed, matchable errors like `GeoReadinessError` | `anyhow` for domain types (loses the match arms) |
| App wiring errors | **anyhow 1.0** | Ergonomic `?` in the builder's composition root | `thiserror` everywhere (ceremony where it isn't needed) |
| gRPC (optional) | **Tonic 0.12 / Prost 0.13** | Schema can generate a gRPC surface, feature-gated off by default | REST-only (loses the option) |
| Codegen source | **Backbone schema YAML** | One SSoT generates entities, DTOs, repos, services, handlers, migrations | Hand-written layers (breaks regeneration) |

## The load-bearing decisions

### PostgreSQL with a dedicated `geo` schema
Every table is schema-qualified (`geo.countries`, `geo.subdistricts`, …). The schema *is* the module
boundary at the database level: geo owns everything under `geo.`, and no other module's migrations
touch it. Cross-module references point at `geo.Subdistrict.id` as a value — never as a database
foreign key — so the two schemas can be migrated independently. This is the physical expression of
the [logical-FK boundary](philosophy.md#the-worldview).

### SQLx over an ORM
Queries are checked against the database at compile time, so a column rename in a migration breaks
the build rather than production. The one hand-written query in the module — the ancestor-drift
count in [`geo_readiness.rs`](../src/application/service/geo_readiness.rs) — is raw SQL precisely
because it expresses a cross-level invariant (does each row's denormalized ancestor match its parent
chain?) that a generic repository cannot. Simple CRUD, by contrast, never touches SQL by hand: it
goes through `GenericCrudRepository`.

### Framework crates from git, pinned by convention
`backbone-core` (with the `postgres` feature), `backbone-orm`, `backbone-auth`, and
`backbone-messaging` are pulled from `github.com/faridlab/backbone-framework`, `branch = "main"`.
For reproducible builds, pin a `tag`/`rev` instead of `branch` (noted in `Cargo.toml`). These crates
supply the generic machinery — `GenericCrudService`, `GenericCrudRepository`, `BackboneCrudHandler`
— that the generated code is thin over. Geo writes almost none of that itself; it *configures* it.

### Feature flags default to off
`events`, `auth`, `grpc`, `openapi`, `validation` are all opt-in and default-empty. Geo is
read-only reference data, so it needs none of them for its core job. They exist so a consumer that
wants, say, a gRPC read surface can turn it on without a fork.

## Why the stack stays small

Geo has **no cache layer, no message bus, no background jobs, no event store** — all of which the
canonical module tree has *slots* for. It doesn't need them: reference data is read-mostly, changes
in bulk offline, and is small enough to serve straight from indexed Postgres. Adding any of those
would be complexity paid for a problem geo doesn't have. The empty optional layers are documented in
the [Architecture](architecture.md) so a maintainer knows where they *would* go, not that they
*should* be added.
