-- Down: drop geo.subdistricts table
DROP TABLE IF EXISTS geo.subdistricts CASCADE;
DROP FUNCTION IF EXISTS geo.subdistricts_audit_timestamp() CASCADE;
