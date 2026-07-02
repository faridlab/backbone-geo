<!-- Reader: Evaluator · Mode: Explanation -->
# Philosophy & Motivation

Geo exists so that **one authoritative administrative hierarchy** is shared by every module that
touches an address — and so that no two modules ever disagree about what "Jawa Barat" or
"Bandung" means.

## The problem

Party addresses, shipping, tax (per-region rates), and point-of-sale all need to resolve and
validate an address against a canonical list of administrative regions. If each module carries its
own free-text province/city strings, they never reconcile: one writes "DKI Jakarta", another
"Jakarta", a third "DKI JAKARTA", and a report that joins them silently drops rows. The bug is
invisible until someone tries to roll revenue up by province.

The fix is a single reference set that every module points at by id. An address stops being free
text and becomes a `subdistrict_id` — a foreign reference into one shared, correct hierarchy.

## The worldview

Three convictions shape every decision in this module.

1. **Reference data is not a transactional domain.** The administrative hierarchy of a country
   changes on the order of *years*, by government decree, in bulk — not per-request, per-user. So
   geo is **loaded from datasets, not authored through an API**, and it is served **read-only**.
   There is no create/update/delete path in the production mount. This single stance removes an
   entire class of concerns: no write validation, no optimistic locking, no audit trail on writes,
   no "who changed Bandung's name" questions.

2. **Denormalize for the read you actually do.** The dominant query is "give me everything under
   region X". So every row carries its **full ancestor chain** — a subdistrict stores its district,
   city, province, and country ids directly. Filtering by any ancestor is a single indexed
   equality, never a four-level join. This is a read model, tuned for its one job.

3. **Boundaries are logical, not physical.** Geo owns the `geo` Postgres schema and nothing reaches
   into another module's tables. Consumers reference `geo.Subdistrict.id` as a **logical foreign
   key** — a value they store and resolve — but there is **no database FK and no Cargo dependency
   edge** across the boundary. Geo never imports a consumer; a consumer never couples its build to
   geo's internals. See [ADR-001](adr/ADR-001-geo-reference-boundary.md).

## What geo deliberately does **not** do

Stating the non-goals is what makes the module trustworthy — it is small on purpose.

- **It does not store addresses.** A person's or company's address lives in `backbone-party`, which
  references `geo.Subdistrict.id` / `City.id`. Geo holds *regions*, not *who lives where*.
- **It is not a GIS engine.** No geocoding or reverse-geocoding, no boundary polygons, no
  distance/radius search. Coordinates exist as plain `latitude`/`longitude` columns for display,
  not for spatial queries.
- **It does not own shipping zones or tax rates.** Those are overlays owned by a shipping module and
  by `backbone-tax-id`, keyed off geo ids.
- **It is not writable over HTTP.** Corrections happen out-of-band via a seed re-load into a clean
  schema — never through the API.

Everything on this list is a real, considered *parking lot* (see the ADRs), not an oversight. Any
of it can be added later when a consumer genuinely needs it, without changing the core model.

## Why this is net-new

ERPNext — the ERP this workspace draws lineage from — ships **no Indonesian administrative set**.
The full four-level *wilayah* (provinsi → kota/kabupaten → kecamatan → kelurahan/desa), with postal
codes and coordinates, is ~84,000 leaf nodes of public reference data that had to be adopted and
shipped. That dataset, plus the read-tuned denormalized model, is the differentiated value of this
module. See [Background](background.md).

## The test of success

- One shared, correct Indonesian hierarchy that every module resolves against by id.
- Read-only over HTTP; reference data is never mutated through the API.
- A dedicated `geo` schema, zero horizontal Cargo edges, referenced only by logical FK.
- A fresh tenant **cannot** boot "healthy" while empty — [readiness](adr/ADR-002-seed-lifecycle-and-readiness.md)
  fails until the seed is loaded and internally consistent.
