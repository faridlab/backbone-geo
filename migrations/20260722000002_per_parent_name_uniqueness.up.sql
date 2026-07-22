-- Enforce per-parent name uniqueness on the geo hierarchy (ADR/Phase 3 geo review F1).
-- Province names unique within a country, city names unique within a province, district
-- names unique within a city. Country (isocode) and subdistrict were already unique.
-- Partial on soft-delete (metadata->>'deleted_at') to match the existing pattern.
-- Reference data is seeded and clean, so these constraints apply without backfill.

CREATE UNIQUE INDEX IF NOT EXISTS idx_provinces_country_id_name
    ON geo.provinces (country_id, name)
    WHERE (metadata ->> 'deleted_at') IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_cities_province_id_name
    ON geo.cities (province_id, name)
    WHERE (metadata ->> 'deleted_at') IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_districts_city_id_name
    ON geo.districts (city_id, name)
    WHERE (metadata ->> 'deleted_at') IS NULL;
