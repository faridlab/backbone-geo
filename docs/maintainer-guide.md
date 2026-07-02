<!-- Reader: Maintainer · Mode: How-to -->
# Maintainer Guide

How to maintain geo and change it without breaking regeneration. Read
[Architecture](architecture.md) first for the layer map; this page is the task-by-task procedure.

## Before you touch anything

- Read this module's [`CLAUDE.md`](../CLAUDE.md) and the root `metaphor.yaml`.
- Remember the project type: **`module`** — a library crate. The **schema YAML is the source of
  truth**; most `src/` code is generated from it.
- Know the two ways hand-written code survives regeneration:
  1. **`// <<< CUSTOM` … `// END CUSTOM` markers** — content between them is preserved even inside a
     generated file (used in `lib.rs`, `presentation/http/mod.rs`, service `mod.rs`).
  2. **`user_owned` globs** in [`metaphor.codegen.yaml`](../metaphor.codegen.yaml) — whole files the
     generator never reads, merges, or deletes.

Everything else in `src/` is overwritten on the next generate. If you hand-edit a generated file
outside a marker, your change is silently lost.

## Where code goes

| Layer | Holds | May depend on |
|-------|-------|---------------|
| Domain (`src/domain/`) | Entities, repository traits (ports), invariants | nothing |
| Application (`src/application/`) | Service type aliases, DTOs, custom services (`geo_readiness.rs`) | domain |
| Infrastructure (`src/infrastructure/`) | Repository newtypes over `GenericCrudRepository` | domain, application |
| Presentation (`src/presentation/`) | Generated CRUD handlers, custom `guarded_routes.rs` | application |

## Task: add a field to an existing entity

Example — add an alternate-name column to `Subdistrict`.

1. **Edit the schema YAML** — [`schema/models/subdistrict.model.yaml`](../schema/models/subdistrict.model.yaml).
   Add the field with its type and attributes, following the existing entries:
   ```yaml
   alt_name:
     type: string?
     attributes: ["@max(120)"]
     description: "Alternate / local name"
   ```
2. **Validate the schema**: `metaphor schema validate`.
3. **Create the migration** for the new column: `metaphor migration create add_subdistrict_alt_name`,
   then fill the generated `*.up.sql` / `*.down.sql` (`ALTER TABLE geo.subdistricts ADD COLUMN …`).
   Never hand-edit an already-applied migration — write a new one.
4. **Regenerate** the entity, DTOs, repository, service, and handler from the schema:
   `metaphor schema generate` (add `--force` to overwrite generated files; `user_owned` files and
   `// <<< CUSTOM` blocks are always preserved).
5. **Build & test**: `metaphor dev test` (or `metaphor build && metaphor test`).

> The **seed** does not update itself. A new column ships `NULL`/default for the 84k existing rows
> until you re-seed with data for it. That's fine for an optional column; a required one needs the
> seed regenerated too.

## Task: add a new entity

The hierarchy is a fixed five levels, so this is rare — but the procedure is the standard one:

1. Add `schema/models/<entity>.model.yaml` (copy an existing model; keep denormalized ancestor FKs
   consistent with the [FSD](fsd.md) if it sits in the hierarchy).
2. `metaphor make entity <Name>` scaffolds the layer cake; `metaphor migration create create_<name>_table`.
3. **Register the new service in the composition root.** In [`src/lib.rs`](../src/lib.rs), add the
   `pub use application::service::<Name>Service;`, a field on `GeoModule`, and its construction in
   `GeoModuleBuilder::build`. The service construction lines are outside CUSTOM markers only because
   they are *generated for known entities* — confirm the generator wrote them, or add them inside the
   builder's `// <<< CUSTOM` block if hand-wiring.
4. If the entity should be readable over HTTP, add its `create_<entity>_read_routes(...)` to
   `create_guarded_geo_routes` in [`guarded_routes.rs`](../src/presentation/http/guarded_routes.rs)
   (a `user_owned` file — edit it directly).

## Task: change the read surface

The production surface is **read-only by policy**, enforced in one hand-authored file.

- To expose or hide an entity's reads, edit `create_guarded_geo_routes` in `guarded_routes.rs`.
- **Do not** add write routes to the guarded mount. Reference data has no runtime write path; a
  correction is a re-seed. Integrity probe **IGC-2** asserts writes return `405/404` — if you mount a
  write, that test goes red on purpose.
- `all_crud_routes()` (full 12-endpoint CRUD) exists for trusted admin/seed tooling only. Leave
  `routes()` deprecated; don't un-deprecate it.

## Task: refresh or fix the seed

The seed lives in [`migrations/seeds/`](../migrations/seeds/) and is `user_owned`.

- Reload the **whole** set into a clean schema in hierarchy order — never partially. Rows carry fixed
  UUIDs so parent→child links stay self-consistent:
  ```bash
  DATABASE_URL=postgres://... ./migrations/seeds/load_all.sh
  ```
- After any re-seed, `geo_readiness_check(&pool)` must return `Ok`. A partial or hand-edited load
  that leaves a row's denormalized `province_id`/`city_id`/`country_id` disagreeing with its parent
  chain is reported as `AncestorDrift(n)` — fix the data, don't silence the check.

## The readiness contract (for composing services)

A service that mounts geo **must** gate startup/readiness on `geo_readiness_check(&pool)`. This is
part of geo's public contract, not an optional nicety — see
[ADR-002](adr/ADR-002-seed-lifecycle-and-readiness.md) and the [Developer Guide](developer-guide.md).
In CI/prod, set `GEO_REQUIRE_SEED=1` so the smoke test **fails** (not skips) on an empty DB.

## Versioning & release

- `backbone-geo` is `0.1.0`. Bump per conventional commits; keep a changelog if you cut releases.
- The framework crates track `branch = "main"`; pin a `tag`/`rev` in `Cargo.toml` for reproducible
  builds before a release.
- `metaphor build && metaphor test` must be clean.
- Commits: **conventional commits, no Claude signatures, no `Co-Authored-By`** (workspace rule).

## What will break things

- Editing generated code outside `// <<< CUSTOM` markers — silently overwritten next generate.
- Adding a `main.rs` / binary target — geo is a library (`[lib]` only).
- Adding a **database** foreign key from a consumer into `geo.*`, or a **Cargo** dependency edge from
  geo to a consumer — both violate the [logical-FK boundary](philosophy.md#the-worldview).
- Mounting write routes on the production surface, or un-deprecating `routes()`.
- Partially re-seeding — desyncs the denormalized ancestor ids and fails readiness.

## Known drift

Skeleton leftovers — **all cleaned 2026-07-02**, lib/bins/tests verified compiling (`cargo check --offline`):

- **`README.md` (repo root)** — rewritten from the `Example` skeleton into a real geo README that
  points at [`docs/`](README.md).
- **`src/module.rs`** — deleted (dead skeleton `Module`/`ExampleService`; the composition root is
  `GeoModule` in `src/lib.rs`).
- **`src/routes/` and `src/handlers/`** — deleted. Neither was declared in `lib.rs`, so they were
  uncompiled dead code; geo's routing is `guarded_routes.rs` + the per-entity handlers.
- **The whole `Example` vertical** — deleted: `domain/entity/example.rs` (incl. `ExampleStatus`),
  `domain/repositories/example_repository.rs`, `application/service/example_service.rs`,
  `infrastructure/persistence/example_repository_impl.rs`, `presentation/http/example_handler.rs`,
  `seeders/example_seeder.rs`, and the uncompiled `application/dto/` and `application/workflows/`
  dirs (which held only example scaffolding). No compiled `mod.rs` had declared any of these, so no
  declaration edits were needed.

No `Example` identifiers remain in `src/` (only a few `// Example:` prose comments in specification
scaffolding). Empty uncompiled layer dirs (`application/auth`, `application/usecases`, etc.) are
left as intentional slots — see [Architecture § Canonical vs. shipped layout](architecture.md#canonical-vs-shipped-layout).
