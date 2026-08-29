-- Enforce integrity on the denormalized geo ancestor columns, at two levels.
--
-- The schema models always carried country_id/province_id/city_id down the hierarchy for
-- fast filtering, but only the immediate-parent FK was ever constrained — the ancestor
-- columns accepted any UUID and ancestor drift (e.g. a city whose country differs from its
-- province's country) was representable. The schema relations now declare every ancestor
-- (model YAML is the source of truth), and this migration adds the matching constraints:
--
--   1. Existence: plain single-column FKs on each ancestor column (any referenced id
--      must exist). These mirror the DSL-declared relations.
--   2. Chain agreement: composite FKs from each child's (parent_id, ancestors...) tuple
--      to the parent's (id, ancestors...) unique key — so a child can only reference a
--      parent row whose OWN ancestors match. Cross-ancestor drift (all ids real, chain
--      inconsistent) becomes unrepresentable at the database level; the boot-time
--      readiness guard remains as defense-in-depth.
--
-- Reference data is seeded with fixed self-consistent UUIDs and the readiness guard
-- already asserted zero ancestor drift, so these constraints apply without backfill.

-- Level 1 — existence FKs on every denormalized ancestor column.
ALTER TABLE geo.cities
    ADD CONSTRAINT fk_cities_country_id
    FOREIGN KEY (country_id) REFERENCES geo.countries (id);

ALTER TABLE geo.districts
    ADD CONSTRAINT fk_districts_country_id
    FOREIGN KEY (country_id) REFERENCES geo.countries (id);

ALTER TABLE geo.districts
    ADD CONSTRAINT fk_districts_province_id
    FOREIGN KEY (province_id) REFERENCES geo.provinces (id);

ALTER TABLE geo.subdistricts
    ADD CONSTRAINT fk_subdistricts_country_id
    FOREIGN KEY (country_id) REFERENCES geo.countries (id);

ALTER TABLE geo.subdistricts
    ADD CONSTRAINT fk_subdistricts_province_id
    FOREIGN KEY (province_id) REFERENCES geo.provinces (id);

ALTER TABLE geo.subdistricts
    ADD CONSTRAINT fk_subdistricts_city_id
    FOREIGN KEY (city_id) REFERENCES geo.cities (id);

-- Level 2 — parent-side unique keys (each trivially unique because id is the primary
-- key; they exist so the composite child FKs have a referencable target).
ALTER TABLE geo.provinces
    ADD CONSTRAINT uq_provinces_id_country
    UNIQUE (id, country_id);

ALTER TABLE geo.cities
    ADD CONSTRAINT uq_cities_id_province_country
    UNIQUE (id, province_id, country_id);

ALTER TABLE geo.districts
    ADD CONSTRAINT uq_districts_id_city_province_country
    UNIQUE (id, city_id, province_id, country_id);

-- Level 2 — chain-agreement composite FKs.
ALTER TABLE geo.cities
    ADD CONSTRAINT fk_cities_province_country_chain
    FOREIGN KEY (province_id, country_id)
    REFERENCES geo.provinces (id, country_id);

ALTER TABLE geo.districts
    ADD CONSTRAINT fk_districts_city_province_country_chain
    FOREIGN KEY (city_id, province_id, country_id)
    REFERENCES geo.cities (id, province_id, country_id);

ALTER TABLE geo.subdistricts
    ADD CONSTRAINT fk_subdistricts_district_chain
    FOREIGN KEY (district_id, city_id, province_id, country_id)
    REFERENCES geo.districts (id, city_id, province_id, country_id);
