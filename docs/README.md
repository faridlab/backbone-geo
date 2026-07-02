<!-- Reader: All · Mode: Navigation -->
# backbone-geo Handbook

The administrative-geography **reference-master** module for the Metaphor/Backbone workspace.
It owns one thing: the authoritative hierarchy **Country → Province → City → District →
Subdistrict** (Indonesia-first, the full Kemendagri *wilayah* set), served read-only so every
other module resolves and validates addresses against one shared list instead of inventing its own.

New here? Pick your reader:

| You are… | You want… | Start at |
|----------|-----------|----------|
| **Evaluating** whether to depend on geo | Why it exists, what it refuses to do | [Philosophy](philosophy.md) → [Background](background.md) |
| **Building a service** that uses geo | Compose it, mount routes, gate readiness | [Developer Guide](developer-guide.md) |
| **Maintaining / extending** geo | How the machine works, add a field safely | [Architecture](architecture.md) → [Maintainer Guide](maintainer-guide.md) |
| **Contributing** a change | Dev setup, tests, PR rules | [Contributing](contributing.md) |
| **Confused by a term** | One definition, used everywhere | [Glossary](glossary.md) |

## The whole handbook

### Explanation — understand *why*
- **[Philosophy & motivation](philosophy.md)** — the problem, the worldview, the non-goals.
- **[Background & prior art](background.md)** — where the model and seed came from, what ERPNext lacked.
- **[Technology & the "why"](technology.md)** — the stack and the reasoning behind each choice.
- **[Architecture](architecture.md)** — C4 context → containers → the DDD 4 layers → a read traced end-to-end.

### How-to — get a goal done
- **[Developer Guide](developer-guide.md)** — install → quickstart → recipes → configuration → troubleshooting.
- **[Maintainer Guide](maintainer-guide.md)** — add a field/entity without breaking regeneration.
- **[Extension Guide](extension-guide.md)** — the stable public surface a consumer depends on.

### Reference — exact facts
- **[Product Requirements (PRD)](prd.md)** — scope, in/out, success criteria.
- **[Functional Spec (FSD)](fsd.md)** — entities, endpoints, invariants.
- **[Business flows](business-flows/geo.md)** + **[Golden cases (the oracle)](business-flows/golden-cases.md)** — behavior, pinned to tests.
- **[Schema system docs](schema/README.md)** — the YAML DSL that generates this module.
- **[Glossary](glossary.md)** — ubiquitous language.

### Decisions — immutable records
- **[ADR-001 — the geo reference boundary](adr/ADR-001-geo-reference-boundary.md)** — what geo owns and refuses to own.
- **[ADR-002 — seed lifecycle & readiness guard](adr/ADR-002-seed-lifecycle-and-readiness.md)** — why an empty geo is a hard failure.

## The one-minute mental model

- **Reference data, not a transactional domain.** Geo is loaded from SQL seeds; it is served
  read-only. There is no legitimate runtime write path.
- **Schema YAML is the source of truth.** `schema/models/*.model.yaml` generates entities, DTOs,
  repositories, services, handlers, and migrations. Hand-written code lives in `// <<< CUSTOM`
  markers or in `user_owned` files listed in `metaphor.codegen.yaml`.
- **Denormalized hierarchy.** Every row carries its full ancestor chain (a subdistrict knows its
  district, city, province, country ids) so "everything under province X" needs no deep join.
- **Cross-module links are logical FKs.** A consumer stores `geo.Subdistrict.id`; it does **not**
  add a database foreign key or a Cargo dependency edge into geo.
- **Seeding is a deploy step, guarded.** A composing service must gate readiness on
  `geo_readiness_check(&pool)` — an unseeded or drifted geo refuses to serve.

---

> **Doc status (2026-07-02).** Handbook authored against the live schema, source, and CLI.
> Skeleton drift fully cleaned: the repo-root `README.md` was rewritten and the entire `Example`
> scaffold plus the uncompiled `src/module.rs`, `src/routes/`, and `src/handlers/` were removed
> (lib/bins/tests verified compiling). See
> [Maintainer Guide § Known drift](maintainer-guide.md#known-drift).
