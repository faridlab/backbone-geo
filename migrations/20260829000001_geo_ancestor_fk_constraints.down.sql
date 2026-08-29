-- Drop the denormalized-ancestor integrity constraints (reverse of the up migration,
-- in dependency order: child chain FKs, then parent unique keys, then existence FKs).

ALTER TABLE geo.subdistricts DROP CONSTRAINT IF EXISTS fk_subdistricts_district_chain;
ALTER TABLE geo.districts    DROP CONSTRAINT IF EXISTS fk_districts_city_province_country_chain;
ALTER TABLE geo.cities       DROP CONSTRAINT IF EXISTS fk_cities_province_country_chain;

ALTER TABLE geo.districts    DROP CONSTRAINT IF EXISTS uq_districts_id_city_province_country;
ALTER TABLE geo.cities       DROP CONSTRAINT IF EXISTS uq_cities_id_province_country;
ALTER TABLE geo.provinces    DROP CONSTRAINT IF EXISTS uq_provinces_id_country;

ALTER TABLE geo.subdistricts DROP CONSTRAINT IF EXISTS fk_subdistricts_city_id;
ALTER TABLE geo.subdistricts DROP CONSTRAINT IF EXISTS fk_subdistricts_province_id;
ALTER TABLE geo.subdistricts DROP CONSTRAINT IF EXISTS fk_subdistricts_country_id;
ALTER TABLE geo.districts    DROP CONSTRAINT IF EXISTS fk_districts_province_id;
ALTER TABLE geo.districts    DROP CONSTRAINT IF EXISTS fk_districts_country_id;
ALTER TABLE geo.cities       DROP CONSTRAINT IF EXISTS fk_cities_country_id;
