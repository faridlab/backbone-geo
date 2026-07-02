<!-- Reader: Evaluator · Mode: Explanation -->
# Background & Prior Art

Where geo's model and data came from, and why the obvious alternatives fell short.

## The gap in ERPNext

The workspace's ERP lineage traces to ERPNext, which models parties, addresses, tax, and stock but
ships **no Indonesian administrative reference set**. Its address is essentially free text plus a
country. For an Indonesia-first ERP that must resolve to *kelurahan/desa* level — for postal
routing, per-region tax, and government reporting — that is a missing foundation, not a
customization. Geo fills it. This is why the module is *net-new and differentiated* rather than a
re-skin of an existing table.

## Prior art this borrows from

- **The DDD 4-layer module shape** (domain / application / infrastructure / presentation) and the
  **schema-YAML-as-SSoT + regeneration** model are the standard Backbone module conventions,
  documented workspace-wide. Geo does not invent architecture; it is a faithful instance of the
  `module` project type. See [Architecture](architecture.md) and the root/module `CLAUDE.md`.
- **The Kemendagri *wilayah* dataset** — the official Indonesian administrative hierarchy
  (provinsi → kota/kabupaten → kecamatan → kelurahan/desa) with postal codes (*kode pos*) and
  coordinates. This is public, factual reference data.

## Adoption, not invention

The entity model **and** the ~84k-row seed were **adopted from a prior Metaphor service**
(`bersihir-metaphor/apps/bersihir-service`), where the denormalized hierarchy had already been
proven against real address-resolution traffic. Geo lifts that read model into a standalone,
reusable bounded context so the whole workspace — not just one app — can depend on it. The
denormalized ancestor columns are a deliberate inheritance from that service's read tuning, not a
fresh guess. See [ADR-001](adr/ADR-001-geo-reference-boundary.md).

The seed rows carry **fixed UUIDs** for every primary and foreign key, so the parent→child links
are self-consistent and the SQL files load verbatim in hierarchy order. This is what lets the data
be adopted as-is and protected from codegen (`user_owned`) rather than re-derived.

## Alternatives considered and rejected

| Alternative | Why rejected |
|-------------|--------------|
| **Each module keeps its own province/city strings** | Guarantees they never reconcile; joins silently drop rows. The exact failure geo exists to prevent. |
| **A normalized hierarchy (join up four levels per query)** | The dominant query is "everything under region X". Four-level joins on 84k rows for every address picker is the wrong cost. Denormalized ancestor ids make it one indexed equality. |
| **A third-party geocoding service / GIS database (PostGIS)** | Overkill and a runtime dependency. Geo needs *identity and hierarchy*, not spatial math. Coordinates are stored for display only. See [Philosophy § non-goals](philosophy.md#what-geo-deliberately-does-not-do). |
| **Writable admin CRUD over HTTP** | Reference data changes in bulk by decree, not per-request. A write API invites drift and demands validation/audit machinery for a path that should never run. Corrections are a re-seed. |
| **Ship the model empty, load data per-deployment however** | This *was* the first cut, and a maturity council found it HIGH-risk: a fresh tenant boots "healthy" while empty, and every lookup silently returns nothing. Fixed by making the seed a required, guarded lifecycle step — see [ADR-002](adr/ADR-002-seed-lifecycle-and-readiness.md). |

## Country-agnostic, Indonesia-first

The structure is country-agnostic: `Country` sits above the hierarchy, and additional countries can
be seeded later with **no schema change**. The *data* ships Indonesia-first because that is the
concrete need today. "Indonesia-first" is a data decision, not a modeling limitation.
