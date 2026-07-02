# ADR-002: Seed is a lifecycle step, enforced by a readiness guard

**Status**: Accepted — **Applied 2026-07-02**
**Deciders**: Farid (owner), council (module:backbone-geo, focus=maturity, 2026-07-02)
**Related**: ADR-001 (geo reference boundary)

## Context

A maturity-focus council found a HIGH operational defect. The 84k-row *wilayah* seed is **not part
of the migration/deploy lifecycle** — the numbered migrations create empty tables (zero INSERTs) and
the data loads only via an out-of-band shell loop or a detached seeder binary. Worse, the one
guardrail that could catch an empty geo — the seed smoke test — **skipped instead of failing** when
`provinces == 0`. So a green test suite was fully consistent with a production `geo` holding zero
rows.

Because cross-module links into geo are logical FKs only (no DB FK), an empty geo raises no
referential error either — it surfaces as **empty JOINs at read time**: blank address dropdowns,
null tax/shipping jurisdiction, no 500. That is the *default first-boot outcome* for a new tenant,
not an attacker edge case. A secondary MEDIUM: the denormalized ancestor ids
(`subdistrict.province_id` etc.) have no FK/CHECK asserting they match the parent chain, so a partial
re-seed or hand-correction can silently desync roll-ups.

## Decision

1. **A readiness guard makes empty/drifted geo a hard failure.** `geo_readiness_check(&pool)`
   returns `NotSeeded` when geo has no rows and `AncestorDrift(n)` when any denormalized ancestor id
   disagrees with its parent chain (checked across subdistrict→district→city→province). A composing
   service **must** gate startup/readiness on it, so an unseeded or corrupted geo refuses to serve
   instead of silently returning empty lookups. Hand-authored, `user_owned`
   (`src/application/service/geo_readiness.rs`).
2. **The seed is a required, single-command deploy step.** `migrations/seeds/load_all.sh` loads all
   five files in hierarchy order; the seeds README marks it non-optional and points at the readiness
   gate.
3. **The smoke test fails, not skips, in CI/prod.** With `GEO_REQUIRE_SEED=1` (set in CI/prod), an
   unseeded geo turns the suite red; local dev without the flag still skips. Proven: an empty DB with
   the flag fails `readiness_and_ancestor_consistency`.
4. **Ancestor drift is detected at readiness** (the MEDIUM finding) rather than via a heavy
   per-row insert trigger — geo has no runtime writes, and a trigger would slow the bulk seed; the
   readiness check catches any post-load desync. Proven by `readiness_detects_ancestor_drift`.

## Consequences

- A fresh tenant can no longer boot "healthy" while empty — readiness fails until the seed loads.
- Two new golden tests pin it: readiness passes + 0 drift on the seeded DB; drift is detected on a
  crafted bad row; the smoke test fails-on-empty under the flag.
- Residual / parking lot (per the council): official **BPS/Kemendagri `kode_wilayah`** columns
  (add when a DJP/government/logistics consumer lands — the seed ships names + UUIDs only); a
  DB-level ancestor CHECK/trigger (readiness covers it for now); geocoding/boundary polygons
  (deferred); `all_crud_routes()` ergonomics (already sealed workspace-wide — read-only is the
  documented mount, `routes()` is `#[deprecated]`, integrity probes assert writes are 405/404).
