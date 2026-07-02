-- Down: drop geo.countries table
DROP TABLE IF EXISTS geo.countries CASCADE;
DROP FUNCTION IF EXISTS geo.countries_audit_timestamp() CASCADE;
