# Graph Report - backbone-geo  (2026-07-22)

## Corpus Check
- 189 files · ~1,447,065 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2108 nodes · 3534 edges · 148 communities (142 shown, 6 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 35 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `579943a4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Uuid
- Subdistrict
- District
- City
- ApiVersion
- Province
- schema/README.md
- PostgresEventStore
- Schema Error Codes Reference
- Recent Generator Changes (Phases 1–10)
- AuditMetadata
- city_dto.rs
- OpenAPI Schema Generation
- auth/mod.rs
- TimestampsBuilder
- MetadataBuilder
- geo_golden_cases.rs
- PostgresSnapshotStore
- SubdistrictResponseDto
- district_dto.rs
- province_dto.rs
- TestResult
- ApiTest
- AppState
- handlers.rs
- Module Integration Schema
- Backbone Schema System
- config/generated.rs
- Country
- CountryId
- CommonUtils
- GeoModule
- Validation Attributes Quick Reference
- create_city_routes
- create_district_routes
- create_province_routes
- create_subdistrict_routes
- repositories/city_repository.rs
- repositories/district_repository.rs
- repositories/province_repository.rs
- repositories/subdistrict_repository.rs
- Model Schema YAML Rules & Format
- Workflow Schema YAML Rules & Format
- repositories/country_repository.rs
- province.rs
- routes/generated.rs
- CityApiTest
- DistrictApiTest
- SubdistrictApiTest
- Metaphor Domain Module
- String
- validator/mod.rs
- services.rs
- SeedCitySeeder
- SeedCountrySeeder
- SeedDistrictSeeder
- SeedProvinceSeeder
- SeedSubdistrictSeeder
- Developer Guide
- create_guarded_geo_routes
- golden-cases.md
- Maintainer Guide
- Type System Reference
- CityRepository
- CountryRepository
- DistrictRepository
- ProvinceRepository
- SubdistrictRepository
- GeoModuleBuilder
- Architecture
- Expression Syntax
- Hook Schema YAML Rules & Format
- Field Attributes
- create_country_routes
- ProvinceApiTest
- Business Flow — Address Resolution (Geo reference)
- Common Mistakes
- Common Mistakes
- Custom Types
- CountryError
- Contributing to backbone-geo
- Common Mistakes
- Quick Reference Checklist
- The load-bearing decisions
- CountryApiTest
- backbone-geo — FSD
- The whole handbook
- Action Steps
- Primitive Types
- Background & Prior Art
- Philosophy & Motivation
- backbone-geo — PRD
- Quick Reference Checklist
- Special Types
- Transitions
- Permissions (RBAC)
- Triggers
- Triggers
- Shared Type Composition
- ADR-001: The geo reference bounded context
- ADR-002: Seed is a lifecycle step, enforced by a readiness guard
- backbone-geo — Extension Guide
- Attribute-Based Access Control (ABAC)
- State Actions
- State Machines
- Validation Rules
- Per-Model Generator Filtering
- Field Definitions
- Field Types
- Relations
- Condition Steps
- Loop Steps
- Step Transitions
- Sub-Workflow Composition (Recommended Pattern)
- Terminal Steps
- Value Objects & Typed IDs
- Nullability
- Type Mappings
- backbone-geo
- Soft Delete & Audit Metadata
- Enums
- Indexes
- Compensation (Rollback)
- Configuration
- Expression Syntax
- Context Variables
- Wait Steps
- Geo reference seed data
- seeder.rs
- Seeder
- load_all.sh
- workflows/README.md

## God Nodes (most connected - your core abstractions)
1. `Subdistrict` - 35 edges
2. `AuditMetadata` - 34 edges
3. `City` - 30 edges
4. `District` - 30 edges
5. `Province` - 30 edges
6. `Country` - 29 edges
7. `Workflow Schema YAML Rules & Format` - 25 edges
8. `Uuid` - 23 edges
9. `TestResult` - 22 edges
10. `ApiVersion` - 21 edges

## Surprising Connections (you probably didn't know these)
- `readiness_and_ancestor_consistency()` --calls--> `ancestor_drift_count()`  [INFERRED]
  tests/geo_golden_cases.rs → src/application/service/geo_readiness.rs
- `readiness_detects_ancestor_drift()` --calls--> `ancestor_drift_count()`  [INFERRED]
  tests/geo_golden_cases.rs → src/application/service/geo_readiness.rs
- `readiness_and_ancestor_consistency()` --calls--> `geo_readiness_check()`  [INFERRED]
  tests/geo_golden_cases.rs → src/application/service/geo_readiness.rs
- `module()` --references--> `GeoModule`  [EXTRACTED]
  tests/integrity_probes.rs → src/lib.rs
- `guarded_reads_are_exposed()` --calls--> `create_guarded_geo_routes()`  [INFERRED]
  tests/integrity_probes.rs → src/presentation/http/guarded_routes.rs

## Import Cycles
- 2-file cycle: `src/domain/entity/district.rs -> src/domain/entity/mod.rs -> src/domain/entity/district.rs`
- 2-file cycle: `src/domain/entity/city.rs -> src/domain/entity/mod.rs -> src/domain/entity/city.rs`
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/subdistrict.rs -> src/domain/entity/mod.rs`
- 2-file cycle: `src/domain/entity/country.rs -> src/domain/entity/mod.rs -> src/domain/entity/country.rs`
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/province.rs -> src/domain/entity/mod.rs`

## Communities (148 total, 6 thin omitted)

### Community 0 - "Uuid"
Cohesion: 0.07
Nodes (57): CityId, Clone, CountryId, Debug, DistrictId, ProvinceId, Entity, CityCreatedEvent (+49 more)

### Community 1 - "Subdistrict"
Cohesion: 0.06
Nodes (25): AsRef, DateTime, Decimal, Deref, Display, EntityRepoMeta, Err, Formatter (+17 more)

### Community 2 - "District"
Cohesion: 0.06
Nodes (24): District, DistrictBuilder, DistrictId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 3 - "City"
Cohesion: 0.06
Nodes (24): City, CityBuilder, CityId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 4 - "ApiVersion"
Cohesion: 0.06
Nodes (40): HeaderMap, Next, Request, S, ApiVersion, RenamedField, RenamedField<T>, Option (+32 more)

### Community 5 - "Province"
Cohesion: 0.07
Nodes (24): Province, ProvinceBuilder, ProvinceId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 6 - "schema/README.md"
Cohesion: 0.06
Nodes (42): Application Layer, Bounded Context Model, Data Flow, Domain Layer, Generated File Layout, Infrastructure Layer, Layer Overview, Module Boundaries (+34 more)

### Community 7 - "PostgresEventStore"
Cohesion: 0.09
Nodes (30): Box, Item, Pin, EventEnvelope, EventEnvelope<T>, EventMetadata, DateTime, HashMap (+22 more)

### Community 8 - "Schema Error Codes Reference"
Cohesion: 0.04
Nodes (49): 1. "Unknown type" errors, 2. YAML indentation errors, 3. Circular reference detection, Common Issues, E001: Lexer Error, E002: Syntax Error, E003: Unexpected Token, E004: Unexpected End of File (+41 more)

### Community 9 - "Recent Generator Changes (Phases 1–10)"
Cohesion: 0.04
Nodes (45): After Generation, API Layer (5), Business Logic (12), CLI Reference, Code Generation Reference, Common Issues, CQRS / Projection, Custom Block Markers (in `.rs` files) (+37 more)

### Community 10 - "AuditMetadata"
Cohesion: 0.09
Nodes (25): AuditMetadata, DateTime, Option, Self, Utc, Uuid, Country, CountryListResponseDto (+17 more)

### Community 11 - "city_dto.rs"
Cohesion: 0.09
Nodes (29): City, CityListResponseDto, CityResponseDto, CitySummaryDto, CreateCityDto, PatchCityDto, ApplyUpdateDto, City (+21 more)

### Community 12 - "OpenAPI Schema Generation"
Cohesion: 0.06
Nodes (36): 1. Keep Custom Endpoints Separate, 2. Use Descriptive Operation IDs, 3. Document Business Rules, 4. Version Your API, API Gateway (Kong), Attribute to OpenAPI Constraint, Best Practices, CLI Commands (+28 more)

### Community 13 - "auth/mod.rs"
Cohesion: 0.07
Nodes (25): CityPolicy, AuthContext, City, ResourceAction, ResourcePolicy, CountryPolicy, AuthContext, Country (+17 more)

### Community 14 - "TimestampsBuilder"
Cohesion: 0.13
Nodes (16): Actors, ActorsBuilder, Default, Option, Result, Self, String, Uuid (+8 more)

### Community 15 - "MetadataBuilder"
Cohesion: 0.16
Nodes (11): Metadata, MetadataBuilder, DateTime, Error, Option, Result, Self, String (+3 more)

### Community 16 - "geo_golden_cases.rs"
Cohesion: 0.13
Nodes (22): ancestor_drift_count(), geo_readiness_check(), GeoReadinessError, Display, Error, Formatter, From, PgPool (+14 more)

### Community 17 - "PostgresSnapshotStore"
Cohesion: 0.13
Nodes (17): PostgresSnapshotStore, DateTime, Default, Into, Option, PgPool, Result, Self (+9 more)

### Community 18 - "SubdistrictResponseDto"
Cohesion: 0.17
Nodes (20): CreateSubdistrictDto, PatchSubdistrictDto, ApplyUpdateDto, DateTime, Decimal, From, FromCreateDto, Option (+12 more)

### Community 19 - "district_dto.rs"
Cohesion: 0.17
Nodes (19): CreateDistrictDto, District, DistrictListResponseDto, DistrictResponseDto, DistrictSummaryDto, PatchDistrictDto, ApplyUpdateDto, DateTime (+11 more)

### Community 20 - "province_dto.rs"
Cohesion: 0.17
Nodes (19): CreateProvinceDto, PatchProvinceDto, Province, ProvinceListResponseDto, ProvinceResponseDto, ProvinceSummaryDto, ApplyUpdateDto, DateTime (+11 more)

### Community 21 - "TestResult"
Cohesion: 0.21
Nodes (10): G, TestResult, create_and_get_id(), CrudTestConfig, GenericCrudTest, GenericCrudTest<G>, Option, Self (+2 more)

### Community 22 - "ApiTest"
Cohesion: 0.24
Nodes (12): Client, RequestBuilder, ApiResponse, ApiTest, Error, HashMap, Into, Option (+4 more)

### Community 23 - "AppState"
Cohesion: 0.22
Nodes (13): health_check(), IntoResponse, AppState, AppStateBuilder, Arc, CityService, CountryService, DistrictService (+5 more)

### Community 24 - "handlers.rs"
Cohesion: 0.13
Nodes (15): CityEvent, CountryEvent, DistrictEvent, ProvinceEvent, CityEventHandler, EventHandler, CountryEventHandler, EventHandler (+7 more)

### Community 25 - "Module Integration Schema"
Cohesion: 0.09
Nodes (22): Cross-Module Event Subscriptions, Cross-Module Foreign Keys, Current `external_imports` Syntax, Dependency Rules, Event Flow Diagram, Event Subscriptions, Exports, File Locations (+14 more)

### Community 26 - "Backbone Schema System"
Cohesion: 0.09
Nodes (22): 1. Define Your Model, 2. Define Entity Hook, 3. Generate Everything, Architecture, Backbone Schema System, Design Principles, Documentation Map, Example: Creating a New Entity (+14 more)

### Community 27 - "config/generated.rs"
Cohesion: 0.19
Nodes (17): Path, DatabaseConfig, expand_env_vars(), FeaturesConfig, GeoModuleConfig, LoggingConfig, merge_yaml(), MetricsConfig (+9 more)

### Community 28 - "Country"
Cohesion: 0.20
Nodes (7): Country, DateTime, EntityRepoMeta, Option, PersistentEntity, Utc, Uuid

### Community 29 - "CountryId"
Cohesion: 0.13
Nodes (12): CountryId, AsRef, Deref, Display, Err, Formatter, From, FromStr (+4 more)

### Community 30 - "CommonUtils"
Cohesion: 0.18
Nodes (8): CommonUtils, CountryTestData, Value, Send, Sync, TestDataGenerator, ProvinceTestData, Value

### Community 31 - "GeoModule"
Cohesion: 0.24
Nodes (15): GeoModule, Arc, CityService, CountryService, DistrictService, ProvinceService, Router, SubdistrictService (+7 more)

### Community 32 - "Validation Attributes Quick Reference"
Cohesion: 0.11
Nodes (18): Array, Conditional, Cross-Field, Database, Date & Time, Documentation, Enum & Choice, File & Image (+10 more)

### Community 33 - "create_city_routes"
Cohesion: 0.20
Nodes (15): CityError, create_city_read_routes(), create_city_routes(), create_city_write_routes(), create_protected_city_routes(), A, Arc, CityService (+7 more)

### Community 34 - "create_district_routes"
Cohesion: 0.20
Nodes (15): create_district_read_routes(), create_district_routes(), create_district_write_routes(), create_protected_district_routes(), DistrictError, A, Arc, DistrictService (+7 more)

### Community 35 - "create_province_routes"
Cohesion: 0.20
Nodes (15): create_protected_province_routes(), create_province_read_routes(), create_province_routes(), create_province_write_routes(), ProvinceError, A, Arc, From (+7 more)

### Community 36 - "create_subdistrict_routes"
Cohesion: 0.20
Nodes (15): create_protected_subdistrict_routes(), create_subdistrict_read_routes(), create_subdistrict_routes(), create_subdistrict_write_routes(), A, Arc, From, IntoResponse (+7 more)

### Community 37 - "repositories/city_repository.rs"
Cohesion: 0.13
Nodes (12): CityFilter, CityPaginatedResult, CityPaginationParams, CityRepository, City, Option, Self, Send (+4 more)

### Community 38 - "repositories/district_repository.rs"
Cohesion: 0.13
Nodes (12): DistrictFilter, DistrictPaginatedResult, DistrictPaginationParams, DistrictRepository, District, Option, Self, Send (+4 more)

### Community 39 - "repositories/province_repository.rs"
Cohesion: 0.13
Nodes (12): ProvinceFilter, ProvincePaginatedResult, ProvincePaginationParams, ProvinceRepository, Option, Province, Self, Send (+4 more)

### Community 40 - "repositories/subdistrict_repository.rs"
Cohesion: 0.13
Nodes (12): Option, Self, Send, String, Subdistrict, Sync, Uuid, Vec (+4 more)

### Community 41 - "Model Schema YAML Rules & Format"
Cohesion: 0.12
Nodes (16): Complete Structure, Complete Structure, Definition, Definition in index.model.yaml, Domain Entities, Enhanced Entity Definition, Entity Model Files, File Organization (+8 more)

### Community 42 - "Workflow Schema YAML Rules & Format"
Cohesion: 0.12
Nodes (16): Basic Parallel Structure, Complete Template, File Organization, Human Task Steps, Join Strategies, Naming Conventions, Overview, Overview (+8 more)

### Community 43 - "repositories/country_repository.rs"
Cohesion: 0.13
Nodes (11): CountryFilter, CountryPaginatedResult, CountryPaginationParams, CountryRepository, Country, Option, Self, Send (+3 more)

### Community 45 - "routes/generated.rs"
Cohesion: 0.30
Nodes (14): city_routes(), configure_routes(), country_routes(), district_routes(), HttpServices, province_routes(), Arc, CityService (+6 more)

### Community 46 - "CityApiTest"
Cohesion: 0.20
Nodes (8): CityApiTest, CityTestData, Default, Self, String, Value, Vec, test_city_crud()

### Community 47 - "DistrictApiTest"
Cohesion: 0.20
Nodes (8): DistrictApiTest, DistrictTestData, Default, Self, String, Value, Vec, test_district_crud()

### Community 48 - "SubdistrictApiTest"
Cohesion: 0.20
Nodes (8): Default, Self, String, Value, Vec, SubdistrictApiTest, SubdistrictTestData, test_subdistrict_crud()

### Community 49 - "Metaphor Domain Module"
Cohesion: 0.14
Nodes (13): Anti-patterns, Common tasks, Deeper knowledge (load on demand), Four-layer folder cheatsheet, Golden path, graphify, Key files to read before editing, Metaphor Domain Module (+5 more)

### Community 51 - "String"
Cohesion: 0.20
Nodes (5): CountryBuilder, HashMap, Id, String, Value

### Community 52 - "validator/mod.rs"
Cohesion: 0.15
Nodes (8): CityValidator, CountryValidator, DistrictValidator, city_validator(), country_validator(), district_validator(), subdistrict_validator(), SubdistrictValidator

### Community 53 - "services.rs"
Cohesion: 0.18
Nodes (9): R, ExportSummary, GeoQueryService, GeoQueryServiceImpl, GeoQueryServiceImpl<R>, Arc, Self, Send (+1 more)

### Community 54 - "SeedCitySeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedCitySeeder

### Community 55 - "SeedCountrySeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedCountrySeeder

### Community 56 - "SeedDistrictSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedDistrictSeeder

### Community 57 - "SeedProvinceSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedProvinceSeeder

### Community 58 - "SeedSubdistrictSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedSubdistrictSeeder

### Community 59 - "Developer Guide"
Cohesion: 0.17
Nodes (12): Configuration, Developer Guide, How do I build a cascading address picker?, How do I make my service refuse to boot on empty geo?, How do I refresh the reference data?, How do I resolve an address to its full chain?, How do I store a resolved address on my own entity?, Install (+4 more)

### Community 60 - "create_guarded_geo_routes"
Cohesion: 0.32
Nodes (10): create_guarded_geo_routes(), Router, StatusCode, guarded_reads_are_exposed(), guarded_writes_are_not_exposed(), module(), pool(), PgPool (+2 more)

### Community 61 - "golden-cases.md"
Cohesion: 0.18
Nodes (9): Geo — Golden Cases (the oracle), Hierarchy (`tests/geo_golden_cases.rs`), Loaded seed (reference), Read-only surface (`tests/integrity_probes.rs`), Concepts, Framework / codegen terms, Glossary — Ubiquitous Language, The hierarchy (+1 more)

### Community 62 - "Maintainer Guide"
Cohesion: 0.18
Nodes (11): Before you touch anything, Known drift, Maintainer Guide, Task: add a field to an existing entity, Task: add a new entity, Task: change the read surface, Task: refresh or fix the seed, The readiness contract (for composing services) (+3 more)

### Community 63 - "Type System Reference"
Cohesion: 0.18
Nodes (11): Arrays, Automatic Coercion, Automatic PascalCase Conversion, Collection Types, Explicit Casting, Maps, Next Steps, Table of Contents (+3 more)

### Community 65 - "CityRepository"
Cohesion: 0.22
Nodes (8): CityRepository, City, Deref, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 66 - "CountryRepository"
Cohesion: 0.22
Nodes (8): CountryRepository, Country, Deref, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 67 - "DistrictRepository"
Cohesion: 0.22
Nodes (8): DistrictRepository, Deref, District, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 68 - "ProvinceRepository"
Cohesion: 0.22
Nodes (8): ProvinceRepository, Deref, GenericCrudRepository, PgPool, Province, Self, SoftDelete, Target

### Community 69 - "SubdistrictRepository"
Cohesion: 0.22
Nodes (8): Deref, GenericCrudRepository, PgPool, Self, SoftDelete, Subdistrict, Target, SubdistrictRepository

### Community 70 - "GeoModuleBuilder"
Cohesion: 0.29
Nodes (6): GeoModuleBuilder, Default, Option, PgPool, Result, Self

### Community 71 - "Architecture"
Cohesion: 0.20
Nodes (10): 1. Context, 2. Containers, 3. Components / modules — the DDD 4 layers, 4. Data & control flow — resolve an address (read), 5. The seed & readiness path (out-of-band), Architecture, Canonical vs. shipped layout, Key decisions (+2 more)

### Community 72 - "Expression Syntax"
Cohesion: 0.20
Nodes (10): Aggregate Functions, Collection Operations, Comparison Operators, Date/Time, Expression Syntax, Field References, Logical Operators, Null Handling (+2 more)

### Community 73 - "Hook Schema YAML Rules & Format"
Cohesion: 0.20
Nodes (10): Basic Computed Fields, Complete Template, Computed Field Rules, Computed Fields, File Organization, Hook File Structure, Hook Schema YAML Rules & Format, Naming Conventions (+2 more)

### Community 74 - "Field Attributes"
Cohesion: 0.20
Nodes (10): Array Validation, Choice Validation, Cross-Field Validation, Date/Time Validation, Field Attributes, Identity & Keys, Numeric Validation, Required & Defaults (+2 more)

### Community 75 - "create_country_routes"
Cohesion: 0.47
Nodes (8): create_country_read_routes(), create_country_routes(), create_country_write_routes(), create_protected_country_routes(), A, Arc, CountryService, Router

### Community 76 - "ProvinceApiTest"
Cohesion: 0.29
Nodes (6): ProvinceApiTest, Default, Self, String, Vec, test_province_crud()

### Community 77 - "Business Flow — Address Resolution (Geo reference)"
Cohesion: 0.22
Nodes (8): Actors, Browse the hierarchy (read), Business Flow — Address Resolution (Geo reference), Flows, Invariants (DB-enforced), Load / refresh reference data (admin, out-of-band), Not here, Resolve an address to its chain

### Community 78 - "Common Mistakes"
Cohesion: 0.22
Nodes (9): 1. Missing or Multiple Initial States, 2. Transition to Non-Existent State, 3. Invalid From Array Syntax, 4. Rule Without Condition, 5. Permission Without Role, 6. Trigger with Wrong Name Pattern, 7. Action String Syntax Errors, 8. Computed Field with Side Effects (+1 more)

### Community 79 - "Common Mistakes"
Cohesion: 0.22
Nodes (9): 1. Missing @id on Primary Key, 2. Self-Reference Without @foreign_key, 3. Using @required on Optional Fields, 4. Wrong Enum Default Syntax, 5. Missing Quotes for Array Types, 6. Wrong Collection Name Format, 7. Duplicate Index Names, 8. External Reference Without Import (+1 more)

### Community 80 - "Custom Types"
Cohesion: 0.22
Nodes (9): 1. Type Inheritance with `extends`, 2. Shared Type as JSONB Field, Custom Types, Defining Custom Types, File-Level Types, JSONB Validation, Type Composition, Type with Validation (+1 more)

### Community 81 - "CountryError"
Cohesion: 0.25
Nodes (7): CountryError, From, IntoResponse, Response, Self, ServiceError, String

### Community 82 - "Contributing to backbone-geo"
Cohesion: 0.25
Nodes (8): Commit & PR conventions, Contributing to backbone-geo, Decisions, Dev setup, Lint, Reviewer expectations, Run the tests, The change workflow

### Community 83 - "Common Mistakes"
Cohesion: 0.25
Nodes (8): 1. Unreachable Steps (CRITICAL), 2. Missing Terminal Step, 3. Condition Without Else, 4. Loop Without Terminal for Iterations, 5. Missing on_failure Handler, 6. Referencing Wrong Context, 7. Step Name Conflicts, Common Mistakes

### Community 84 - "Quick Reference Checklist"
Cohesion: 0.25
Nodes (8): Conditions, Context, Loops, Parallel, Quick Reference Checklist, Steps, Transitions, Workflow Structure

### Community 85 - "The load-bearing decisions"
Cohesion: 0.25
Nodes (8): Feature flags default to off, Framework crates from git, pinned by convention, PostgreSQL with a dedicated `geo` schema, SQLx over an ORM, Technology & the "Why", The choices at a glance, The load-bearing decisions, Why the stack stays small

### Community 86 - "CountryApiTest"
Cohesion: 0.36
Nodes (5): CountryApiTest, Default, Self, Vec, test_country_crud()

### Community 87 - "backbone-geo — FSD"
Cohesion: 0.29
Nodes (7): backbone-geo — FSD, Behavior specs, Data, Endpoints, Entities, Integration (logical FKs — no DB FK, no Cargo edge), Non-goals

### Community 88 - "The whole handbook"
Cohesion: 0.29
Nodes (7): backbone-geo Handbook, Decisions — immutable records, Explanation — understand *why*, How-to — get a goal done, Reference — exact facts, The one-minute mental model, The whole handbook

### Community 89 - "Action Steps"
Cohesion: 0.29
Nodes (7): Action Steps, Create Action, Custom Action, Delete Action, Emit Event Action, Query Action, Update Action

### Community 90 - "Primitive Types"
Cohesion: 0.29
Nodes (7): Boolean, Date and Time, Decimal Precision, Identifiers, Numeric Types, Primitive Types, String Types

### Community 91 - "Background & Prior Art"
Cohesion: 0.33
Nodes (6): Adoption, not invention, Alternatives considered and rejected, Background & Prior Art, Country-agnostic, Indonesia-first, Prior art this borrows from, The gap in ERPNext

### Community 92 - "Philosophy & Motivation"
Cohesion: 0.33
Nodes (6): Philosophy & Motivation, The problem, The test of success, The worldview, What geo deliberately does **not** do, Why this is net-new

### Community 93 - "backbone-geo — PRD"
Cohesion: 0.33
Nodes (6): backbone-geo — PRD, Indonesia-first notes, Personas, Problem, Scope, Success criteria

### Community 94 - "Quick Reference Checklist"
Cohesion: 0.33
Nodes (6): Computed Fields, Permissions, Quick Reference Checklist, Rules, State Machine, Triggers

### Community 95 - "Special Types"
Cohesion: 0.33
Nodes (6): Binary Types, File Types, Password Type, Special Types, Structured Types, Validated String Types

### Community 97 - "Transitions"
Cohesion: 0.40
Nodes (5): Basic Transition, Multiple Source States, Transition Condition Expressions, Transition with Guards, Transitions

### Community 98 - "Permissions (RBAC)"
Cohesion: 0.40
Nodes (5): Complete Example, Permission Actions, Permission Expression Variables, Permissions (RBAC), Structure

### Community 99 - "Triggers"
Cohesion: 0.40
Nodes (5): Scheduled Triggers (in index.hook.yaml), Trigger Examples, Trigger Structure, Trigger Types, Triggers

### Community 100 - "Triggers"
Cohesion: 0.40
Nodes (5): Event Trigger, Extract Variables, HTTP Endpoint Trigger, Schedule Trigger (Cron), Triggers

### Community 101 - "Shared Type Composition"
Cohesion: 0.40
Nodes (5): Defining Shared Types, Shared Type Composition, Using Shared Types as Columns (`extends`), Using Shared Types as JSONB Fields, When to Use Which

### Community 102 - "ADR-001: The geo reference bounded context"
Cohesion: 0.50
Nodes (4): ADR-001: The geo reference bounded context, Consequences, Context, Decision

### Community 103 - "ADR-002: Seed is a lifecycle step, enforced by a readiness guard"
Cohesion: 0.50
Nodes (4): ADR-002: Seed is a lifecycle step, enforced by a readiness guard, Consequences, Context, Decision

### Community 104 - "backbone-geo — Extension Guide"
Cohesion: 0.50
Nodes (4): backbone-geo — Extension Guide, Composing into a service, Public / stable surface, Regeneration safety

### Community 105 - "Attribute-Based Access Control (ABAC)"
Cohesion: 0.50
Nodes (4): ABAC Attributes, Attribute-Based Access Control (ABAC), Policies, Resource Policies

### Community 106 - "State Actions"
Cohesion: 0.50
Nodes (4): Action Expression Variables, Action Types, Conditional Actions, State Actions

### Community 107 - "State Machines"
Cohesion: 0.50
Nodes (4): Basic Structure, Critical Rules for States, State Definition, State Machines

### Community 108 - "Validation Rules"
Cohesion: 0.50
Nodes (4): Rule Examples, Rule Structure, Validation Rules, When Clauses

### Community 109 - "Per-Model Generator Filtering"
Cohesion: 0.50
Nodes (4): Available Targets, Module-Level Filtering, Per-Entity Override, Per-Model Generator Filtering

### Community 110 - "Field Definitions"
Cohesion: 0.50
Nodes (4): Field Definitions, Full Syntax, Shorthand Syntax, Special Fields

### Community 111 - "Field Types"
Cohesion: 0.50
Nodes (4): Field Types, Primitive Types, String Format Types, Type Modifiers

### Community 112 - "Relations"
Cohesion: 0.50
Nodes (4): Relation Attributes, Relation Types, Relations, Self-Referencing Relations

### Community 113 - "Condition Steps"
Cohesion: 0.50
Nodes (4): Basic Condition, Complex Conditions, Condition Steps, Multiple Conditions

### Community 114 - "Loop Steps"
Cohesion: 0.50
Nodes (4): Basic Loop Structure, Loop Steps, Loop with Index, Nested Loops

### Community 115 - "Step Transitions"
Cohesion: 0.50
Nodes (4): Basic Transition, Retry Configuration, Setting Context on Transition, Step Transitions

### Community 116 - "Sub-Workflow Composition (Recommended Pattern)"
Cohesion: 0.50
Nodes (4): Example: Order Processing Chain, Rules for Sub-Workflow Chains, Sub-Workflow Composition (Recommended Pattern), Why Decompose

### Community 117 - "Terminal Steps"
Cohesion: 0.50
Nodes (4): Failed Terminal, Success Terminal, Terminal Steps, Terminal with Event Emission

### Community 118 - "Value Objects & Typed IDs"
Cohesion: 0.50
Nodes (4): Composite Value Objects, Typed IDs, Value Objects & Typed IDs, Wrapper Value Objects

### Community 119 - "Nullability"
Cohesion: 0.50
Nodes (4): Default Values, Nullability, Nullability Rules, Required vs Optional

### Community 120 - "Type Mappings"
Cohesion: 0.50
Nodes (4): Schema to PostgreSQL, Schema to Proto, Schema to Rust, Type Mappings

### Community 121 - "backbone-geo"
Cohesion: 0.50
Nodes (4): backbone-geo, Documentation, Layout, Quickstart

### Community 122 - "Soft Delete & Audit Metadata"
Cohesion: 0.67
Nodes (3): Audit Metadata Pattern, Soft Delete, Soft Delete & Audit Metadata

### Community 123 - "Enums"
Cohesion: 0.67
Nodes (3): Enums, Full Enum Definition, Rules for Enums

### Community 124 - "Indexes"
Cohesion: 0.67
Nodes (3): Index Options, Index Types, Indexes

### Community 125 - "Compensation (Rollback)"
Cohesion: 0.67
Nodes (3): Compensation (Rollback), Compensation Structure, Complete Example

### Community 126 - "Configuration"
Cohesion: 0.67
Nodes (3): Complete Config Options, Configuration, Timeout Formats

### Community 127 - "Expression Syntax"
Cohesion: 0.67
Nodes (3): Condition Expressions, Expression Syntax, Template Syntax

### Community 128 - "Context Variables"
Cohesion: 0.67
Nodes (3): Context Variables, Declaring Context, Using Context in Steps

### Community 129 - "Wait Steps"
Cohesion: 0.67
Nodes (3): Wait for Duration, Wait for Event, Wait Steps

## Knowledge Gaps
- **491 isolated node(s):** `load_all.sh script`, `ExportSummary`, `GeoQueryServiceImpl<R>`, `VersionTransform`, `VersionedResponse<T>` (+486 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AuditMetadata` connect `AuditMetadata` to `country.rs`, `Uuid`, `District`, `City`, `Subdistrict`, `Province`, `city_dto.rs`, `province.rs`, `SubdistrictResponseDto`, `district_dto.rs`, `province_dto.rs`, `Country`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **Why does `Type System Reference` connect `Type System Reference` to `Validation Attributes Quick Reference`, `Shared Type Composition`, `schema/README.md`, `Custom Types`, `Value Objects & Typed IDs`, `Nullability`, `Type Mappings`, `Primitive Types`, `Special Types`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **Why does `Model Schema YAML Rules & Format` connect `Model Schema YAML Rules & Format` to `schema/README.md`, `Field Attributes`, `Per-Model Generator Filtering`, `Field Definitions`, `Common Mistakes`, `Field Types`, `Relations`, `Soft Delete & Audit Metadata`, `Enums`, `Indexes`?**
  _High betweenness centrality (0.016) - this node is a cross-community bridge._
- **What connects `load_all.sh script`, `ExportSummary`, `GeoQueryServiceImpl<R>` to the rest of the system?**
  _491 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Uuid` be split into smaller, more focused modules?**
  _Cohesion score 0.07382091592617908 - nodes in this community are weakly interconnected._
- **Should `Subdistrict` be split into smaller, more focused modules?**
  _Cohesion score 0.05821917808219178 - nodes in this community are weakly interconnected._
- **Should `District` be split into smaller, more focused modules?**
  _Cohesion score 0.06153846153846154 - nodes in this community are weakly interconnected._