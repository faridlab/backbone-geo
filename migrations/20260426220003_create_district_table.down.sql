-- Down: drop geo.districts table
DROP TABLE IF EXISTS geo.districts CASCADE;
DROP FUNCTION IF EXISTS geo.districts_audit_timestamp() CASCADE;
