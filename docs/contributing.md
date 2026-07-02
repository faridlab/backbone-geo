<!-- Reader: Contributor · Mode: How-to -->
# Contributing to backbone-geo

How to land a change. For *what goes where* and the regeneration rules, read the
[Maintainer Guide](maintainer-guide.md) first — this page is process.

## Dev setup

1. **Toolchain** — Rust 2021 (stable). This is a library crate (`[lib]` only).
2. **Framework crates** — `backbone-core/orm/auth/messaging` fetch from
   `github.com/faridlab/backbone-framework` (`branch = "main"`). Network access is required for the
   first build.
3. **Database** — a local PostgreSQL for tests. Create the schema and load the seed:
   ```bash
   DATABASE_URL=postgres://root:password@localhost:5432/geo_dev ./migrations/seeds/load_all.sh
   ```
4. **Prefer the `metaphor` CLI** over raw `cargo` — it applies workspace policy. Never run
   `cargo build`/`cargo test` from the *workspace root*; use `metaphor` (inside this project the
   module-scoped commands below are fine).

## The change workflow

Geo is schema-first. The order almost always is:

1. Edit `schema/models/*.model.yaml` (the SSoT) — **not** generated code.
2. `metaphor schema validate`
3. `metaphor migration create <name>` and fill the `*.up.sql`/`*.down.sql`.
4. `metaphor schema generate` (add `--force` to overwrite generated files; `user_owned` files and
   `// <<< CUSTOM` blocks are preserved).
5. Put any hand-written logic in a `user_owned` file or inside `// <<< CUSTOM … // END CUSTOM`.
6. `metaphor dev test` (or `metaphor build && metaphor test`) — green before you push.

## Run the tests

The behavior oracle lives in `user_owned` test files (see [`metaphor.codegen.yaml`](../metaphor.codegen.yaml)):

- `tests/geo_golden_cases.rs` — hierarchy + readiness (**GGC-1…6**).
- `tests/integrity_probes.rs` — the read-only surface (**IGC-1/2**): reads `200`, writes `405/404`.
- `tests/features/geo.feature` — BDD flows.

```bash
metaphor dev test
# CI/prod also set this so the seed smoke test FAILS (not skips) on an empty DB:
GEO_REQUIRE_SEED=1 metaphor dev test
```

If you change behavior, update the matching case in
[`docs/business-flows/golden-cases.md`](business-flows/golden-cases.md) **and** the test in the same
PR — the doc and the oracle move together.

## Lint

```bash
metaphor lint check
```

## Commit & PR conventions

- **Conventional commits** (`feat:`, `fix:`, `docs:`, `refactor:`, …).
- **No signatures.** Do not append `Co-Authored-By`, "Generated with", or any Claude/tool signature
  to commit messages — this is a hard workspace rule.
- Keep commits focused; group by functionality, and don't bundle a large generated diff with an
  unrelated change.
- **PR checklist:**
  - [ ] Schema YAML edited first (if the change is entity-shaped); generated code regenerated, not hand-edited.
  - [ ] New migration added (never edit an applied one); `*.down.sql` present.
  - [ ] Hand-written code is in a `user_owned` file or `// <<< CUSTOM` block.
  - [ ] `metaphor dev test` green, including `GEO_REQUIRE_SEED=1`.
  - [ ] Read-only surface preserved — no write routes on the guarded mount (IGC-2 still green).
  - [ ] Boundary intact — no DB FK into `geo.*`, no Cargo edge from geo to a consumer.
  - [ ] Docs updated: golden cases, and any handbook page the change touches.

## Reviewer expectations

A reviewer will check that the change respects geo's two invariants above all: **read-only reference
data** and the **logical-FK boundary**. A PR that adds an HTTP write path, a cross-module database
FK, or hand-edits generated code outside a marker will be sent back regardless of how well it works.

## Decisions

Architecture-level changes (the boundary, the read-only stance, the seed/readiness contract) are
recorded as ADRs in [`docs/adr/`](adr/). ADRs are **immutable once Accepted** — to change one, write
a new ADR that supersedes it; don't edit the decision in place. Use
[`.claude/skills/framework-handbook/templates/adr-NNNN.md`](../.claude/skills/framework-handbook/templates/adr-NNNN.md)
as the skeleton.
