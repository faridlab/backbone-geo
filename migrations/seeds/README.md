# Geo reference seed data

The Indonesian administrative hierarchy (Kemendagri *wilayah*), adopted from a prior Metaphor
service. Factual public administrative reference data — place names, postal codes, coordinates.

| File | Table | Rows |
|------|-------|------|
| `country_seed.sql` | `geo.countries` | 1 (ID) |
| `province_seed.sql` | `geo.provinces` | 38 |
| `city_seed.sql` | `geo.cities` | 489 |
| `district_seed.sql` | `geo.districts` | 7,274 |
| `subdistrict_seed.sql` | `geo.subdistricts` | 83,741 |

Rows carry **fixed UUIDs** for PK and FK, so the parent→child links are self-consistent and the
files load verbatim. Tables are schema-qualified (`geo.*`).

## Loading — a REQUIRED deploy step

Run the loader (parents first) immediately after migrations, on every fresh tenant DB:

```bash
DATABASE_URL=postgres://... ./migrations/seeds/load_all.sh
```

**This is not optional.** A service must gate startup/readiness on
`backbone_geo::geo_readiness_check(&pool)`, which returns `NotSeeded` until the seed is loaded and
`AncestorDrift(n)` if a partial/hand re-seed left the denormalized ancestor ids inconsistent — so an
unseeded or corrupted geo fails readiness instead of silently serving empty address lookups
(council 2026-07-02, ADR-002). CI/prod also set `GEO_REQUIRE_SEED=1` so the test suite turns red on
an unseeded DB rather than skipping.

The data is a slow-moving reference set; refresh by re-seeding into a clean schema (load the whole
set — never partially). These files are `user_owned` (see `metaphor.codegen.yaml`) so
`metaphor schema generate` never touches them.
