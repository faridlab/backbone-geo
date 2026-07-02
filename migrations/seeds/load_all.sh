#!/usr/bin/env bash
# Load the geo reference data (Indonesia wilayah) in hierarchy order.
# REQUIRED post-migrate step — a service will fail readiness until this runs
# (see backbone_geo::geo_readiness_check). Idempotent-safe on a clean geo schema.
#
#   DATABASE_URL=postgres://... ./migrations/seeds/load_all.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
: "${DATABASE_URL:?set DATABASE_URL}"
for t in country province city district subdistrict; do
  echo ">> seeding ${t}"
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -q -f "${here}/${t}_seed.sql"
done
echo ">> geo seed complete"
