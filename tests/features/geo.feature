# Geo acceptance oracle — backbone-geo
# Flow map:    docs/business-flows/geo.md
# Golden cases: docs/business-flows/golden-cases.md
# Executable truth: tests/geo_golden_cases.rs + tests/integrity_probes.rs

Feature: Administrative geography reference
  In order to resolve and validate addresses across the ERP
  As a consuming module (party, shipping, tax, POS)
  I want to browse the Country → Province → City → District → Subdistrict hierarchy read-only

  Background:
    Given the tenant schema "geo" is migrated and seeded

  @happy-path @module:geo @ggc-1
  Scenario: Resolve an address to its full chain
    Given a subdistrict under a known district, city, province and country
    When I resolve that subdistrict
    Then it returns the district, city, province and country names and its postal code

  @validation @module:geo @ggc-2
  Scenario: Subdistrict names are unique within a district
    Given a subdistrict in a district
    When I add another subdistrict with the same name to the same district
    Then it is rejected

  @read-only @module:geo @igc-2
  Scenario: The reference data cannot be mutated through the API
    When I POST to "/countries" on the guarded routes
    Then the response status is 405 or 404

  @seed @module:geo @ggc-4
  Scenario: The Indonesia reference set is present
    Given the seed is loaded
    Then there are 38 provinces under country "ID"
