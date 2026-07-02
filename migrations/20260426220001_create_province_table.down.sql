-- Down: drop geo.provinces table
DROP TABLE IF EXISTS geo.provinces CASCADE;
DROP FUNCTION IF EXISTS geo.provinces_audit_timestamp() CASCADE;
