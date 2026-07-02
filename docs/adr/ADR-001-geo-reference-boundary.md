# ADR-001: The geo reference bounded context

**Status**: Accepted — **Applied 2026-07-02**
**Deciders**: Farid (owner)
**Related**: workspace `docs/erp/vinsteknik-adoption-map.md`; adopted the model + seed from a prior
Metaphor service (`bersihir-metaphor/apps/bersihir-service`).

## Context

Party addresses, shipping, tax, and POS all need one authoritative administrative hierarchy to
resolve/validate addresses against. Indonesia's *wilayah* (provinsi → kota/kabupaten → kecamatan →
kelurahan/desa) has ~84k leaf nodes; every module inventing its own strings guarantees they never
reconcile. ERPNext ships no Indonesian set, so this is net-new and differentiated.

## Decision

1. **`backbone-geo` owns the administrative hierarchy only** — Country, Province, City, District,
   Subdistrict — as pure **reference data**. It owns no addresses, no shipping zones, no tax rates.
2. **Read-only over HTTP.** `create_guarded_geo_routes` mounts read routes only; the data is loaded
   from SQL seeds and never mutated through the API. (`all_crud_routes()` exists for admin/seed
   tooling but is not the production mount.)
3. **Ancestor FKs are denormalized** down the chain (a subdistrict carries country/province/city/
   district ids), adopted from the prior service — a read model tuned for fast filtering.
4. **Dedicated `geo` Postgres schema**; cross-module references are logical FKs only. A consuming
   module (party) holds an `Address` that references `geo.Subdistrict.id`/`City.id`; geo never
   imports a consumer.
5. **Adopt the seed dataset.** The Indonesian *wilayah* data (public administrative reference —
   place names, postal codes, coordinates) is shipped as `migrations/seeds/*.sql` with fixed UUIDs
   so parent→child links are self-consistent and load verbatim. It is `user_owned` so regeneration
   never touches it.

## Consequences

- One shared, correct hierarchy the whole ERP resolves against; addresses become `subdistrict_id`
  dimensions rather than free text.
- Country-agnostic structure seeded Indonesia-first; other countries can be seeded later without a
  schema change.
- The ~30 MB subdistrict seed lives in the repo — the cost of shipping real reference data, paid
  once and protected from codegen.
- Parking lot: geocoding/reverse-geocoding, boundary polygons, distance search, alternate-name/
  BPS-code columns — added only if a consumer needs them.
