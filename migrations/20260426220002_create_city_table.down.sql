-- Down: drop geo.cities table
DROP TABLE IF EXISTS geo.cities CASCADE;
DROP FUNCTION IF EXISTS geo.cities_audit_timestamp() CASCADE;
