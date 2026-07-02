# Geo Workflows

Geo has **no workflows** — it is read-only reference data with no multi-step sagas and no
lifecycle. The administrative hierarchy is loaded via SQL seeds (`migrations/seeds/`), and served
read-only (`create_guarded_geo_routes`). There is nothing to orchestrate.
