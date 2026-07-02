# backbone-geo — PRD

> Reference-master module. Owns the **administrative geography hierarchy** — Country → Province →
> City → District → Subdistrict — Indonesia-first (the full Kemendagri *wilayah* set down to
> kelurahan/desa with postal codes + coordinates).

## Problem

Party addresses, shipping, tax (per-region rates), and POS all need one authoritative, shared list
of administrative regions to resolve and validate an address against. Without it, every module
invents its own province/city strings and they never reconcile. ERPNext has no Indonesian
administrative set; this is net-new and differentiated.

## Scope

**In:**
- `Country` (isocode, name, phonecode), `Province`, `City`, `District`, `Subdistrict` (postal_code,
  latitude, longitude). Ancestor FKs are denormalized down the chain for fast filtering.
- The full Indonesia reference dataset shipped as SQL seeds (1 country, 38 provinces, 489 cities,
  7,274 districts, 83,741 subdistricts).
- Read-only HTTP access (cascading address pickers, address resolution).

**Out (owned elsewhere / deferred):**
- **Addresses themselves** (a person's/company's address) — `backbone-party` holds an Address that
  references `geo.Subdistrict.id`/`City.id` as logical FKs.
- **Shipping rates / zones** — a shipping module.
- **Tax rates per region** — `backbone-tax-id` overlay.
- Geocoding / reverse-geocoding, polygon boundaries, distance search — not a GIS engine.

## Personas
- **Consuming modules** — resolve an address chain, store `subdistrict_id`/`city_id` as a dimension.
- **Platform admin** — loads/refreshes the seed on a fresh tenant schema.

## Success criteria
- One shared, correct Indonesian administrative hierarchy every module can reference by id.
- Read-only over HTTP; reference data is never mutated through the API (seeded via SQL).
- Dedicated `geo` Postgres schema; zero horizontal Cargo edges; referenced by logical FK only.

## Indonesia-first notes
Full 4-level *wilayah* (provinsi → kota/kabupaten → kecamatan → kelurahan/desa) + kode pos +
coordinates. Country-agnostic structure, seeded Indonesia-first; other countries can be seeded later.
