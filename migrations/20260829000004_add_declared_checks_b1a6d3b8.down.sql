-- Down: drop the declared rule constraints for geo module
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_postal_code_max_length__max_length;
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_district_id_required__required;
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_city_id_required__required;
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_province_id_required__required;
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_country_id_required__required;
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_name_max_length__max_length;
ALTER TABLE IF EXISTS geo.subdistricts DROP CONSTRAINT IF EXISTS ck_subdistricts_name_required__required;
DO $$
BEGIN
    IF to_regclass('geo.uq_subdistricts_name__unique') IS NOT NULL AND to_regclass('geo.idx_subdistricts_district_id_name') IS NULL THEN
        ALTER INDEX geo.uq_subdistricts_name__unique RENAME TO idx_subdistricts_district_id_name;
    END IF;
END $$;

ALTER TABLE IF EXISTS geo.provinces DROP CONSTRAINT IF EXISTS ck_provinces_country_id_required__required;
ALTER TABLE IF EXISTS geo.provinces DROP CONSTRAINT IF EXISTS ck_provinces_name_max_length__max_length;
ALTER TABLE IF EXISTS geo.provinces DROP CONSTRAINT IF EXISTS ck_provinces_name_required__required;
DO $$
BEGIN
    IF to_regclass('geo.uq_provinces_name__unique') IS NOT NULL AND to_regclass('geo.idx_provinces_country_id_name') IS NULL THEN
        ALTER INDEX geo.uq_provinces_name__unique RENAME TO idx_provinces_country_id_name;
    END IF;
END $$;

ALTER TABLE IF EXISTS geo.districts DROP CONSTRAINT IF EXISTS ck_districts_city_id_required__required;
ALTER TABLE IF EXISTS geo.districts DROP CONSTRAINT IF EXISTS ck_districts_province_id_required__required;
ALTER TABLE IF EXISTS geo.districts DROP CONSTRAINT IF EXISTS ck_districts_country_id_required__required;
ALTER TABLE IF EXISTS geo.districts DROP CONSTRAINT IF EXISTS ck_districts_name_max_length__max_length;
ALTER TABLE IF EXISTS geo.districts DROP CONSTRAINT IF EXISTS ck_districts_name_required__required;
DO $$
BEGIN
    IF to_regclass('geo.uq_districts_name__unique') IS NOT NULL AND to_regclass('geo.idx_districts_city_id_name') IS NULL THEN
        ALTER INDEX geo.uq_districts_name__unique RENAME TO idx_districts_city_id_name;
    END IF;
END $$;

ALTER TABLE IF EXISTS geo.countries DROP CONSTRAINT IF EXISTS ck_countries_name_max_length__max_length;
ALTER TABLE IF EXISTS geo.countries DROP CONSTRAINT IF EXISTS ck_countries_name_required__required;
ALTER TABLE IF EXISTS geo.countries DROP CONSTRAINT IF EXISTS ck_countries_isocode_max_length__max_length;
ALTER TABLE IF EXISTS geo.countries DROP CONSTRAINT IF EXISTS ck_countries_isocode_required__required;
DO $$
BEGIN
    IF to_regclass('geo.uq_countries_isocode__unique') IS NOT NULL AND to_regclass('geo.idx_countries_isocode') IS NULL THEN
        ALTER INDEX geo.uq_countries_isocode__unique RENAME TO idx_countries_isocode;
    END IF;
END $$;

ALTER TABLE IF EXISTS geo.cities DROP CONSTRAINT IF EXISTS ck_cities_province_id_required__required;
ALTER TABLE IF EXISTS geo.cities DROP CONSTRAINT IF EXISTS ck_cities_country_id_required__required;
ALTER TABLE IF EXISTS geo.cities DROP CONSTRAINT IF EXISTS ck_cities_name_max_length__max_length;
ALTER TABLE IF EXISTS geo.cities DROP CONSTRAINT IF EXISTS ck_cities_name_required__required;
DO $$
BEGIN
    IF to_regclass('geo.uq_cities_name__unique') IS NOT NULL AND to_regclass('geo.idx_cities_province_id_name') IS NULL THEN
        ALTER INDEX geo.uq_cities_name__unique RENAME TO idx_cities_province_id_name;
    END IF;
END $$;

