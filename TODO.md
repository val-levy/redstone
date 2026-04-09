
# TODO

## First Targets

- apps/web
- apps/gateway
- services/entity-service
- services/search-service
- one database
- maybe one event stream later
  
## First Milestone

  For the first milestone, I’d reduce the effective architecture to:

- apps/web: UI
- apps/gateway: single public API/BFF
- services/entity-service: source of truth for entities
- services/search-service: optional, and only if DB text search is not enough
- PostgreSQL: primary datastore
- one background worker only if truly needed

## Phase 3: Build The Core Domain

Before writing much code, define the core objects in one place. Use docs/architecture/data-model.md for this.

You need:

- Entity
- Alias
- Identifier
- Relationship
- Event
- Source
- Audit/Provenance

For each one, decide:

- required fields
- unique identifiers
- status lifecycle
- who can mutate it
- whether it is immutable or versioned
- what “canonical” means

If this is fuzzy, every service boundary will stay fuzzy.

## Phase 4: Make The Gateway Real

Your gateway is currently a placeholder in apps/gateway/src/main.rs. Turn it into the stable frontend contract first.

Implement:

- health endpoint
- entity create/read/update endpoints
- search endpoint
- error format
- auth stub or API key middleware
- request logging and tracing IDs

Even if the gateway initially talks to only one backend, that is fine. Its job is to stabilize your API surface while internals change.

Phase 5: Build One Service Properly
Build entity-service as the first real backend.

In services/entity-service:

- replace the placeholder binary with an HTTP API or gRPC API
- add database migrations
- implement CRUD for entities
- enforce validation
- add tests around persistence and business rules

Make this service own:

- canonical entity record
- identifiers
- aliases
- basic relationships if needed

Do not spread domain ownership across multiple services early.

## Phase 6: Get Search Working Cheaply

You likely do not need a separate search stack on day one.

Roadmap for search:

1. start with PostgreSQL full-text or trigram search inside entity-service
2. expose it through gateway
3. only split into search-service when indexing/query behavior becomes meaningfully separate

Your current search-service can stay stubbed until that split is justified.

Phase 7: Make The Frontend A Real Internal Tool
Your web app in apps/web/src/app/page.tsx should become an operator-facing interface first, not a polished broad platform.

Build these screens:

- entity search
- entity detail page
- create/edit entity form
- recent activity list
- relationship view
- basic event timeline if needed

That will force clarity in your API and data model faster than more backend scaffolding.

Phase 8: Add Operational Basics Early
Before adding more product breadth, add minimum production discipline:

- structured logs
- migration workflow
- local seed data
- smoke tests
- request tracing
- health/readiness endpoints
- one local dev command that actually works

Right now files like ops/scripts/dev-up.sh and docs/api/openapi.yaml are placeholders. Turn those into real artifacts early.

## Suggested 90-Day Roadmap

Weeks 1-2

- finalize product scope for first vertical slice
- write real data model doc
- choose primary DB and API style
- clean up naming drift (Redstone vs Gotham)
- make local dev actually boot the minimum stack

Weeks 3-4

- implement gateway baseline
- implement entity-service with persistence
- add migrations and seed data
- add CRUD + health endpoints
- add integration tests

Weeks 5-6

- build entity search
- build web app search and detail views
- connect web -> gateway -> entity-service
- add OpenAPI spec for working endpoints

Weeks 7-8

- add relationships and simple event history
- add audit/provenance model
- improve filtering and pagination
- add authentication stub and authorization shape

Weeks 9-10

- evaluate whether search deserves its own service
- if yes, split indexing/query concerns cleanly
- if no, keep it in the core service and move on

Weeks 11-12

- harden ops: observability, smoke tests, deploy path
- decide next highest-value module: resolution, ingestion, or timeline
- only then start the second service expansion

What Not To Do Yet
Avoid these early:

- real microservice fanout across all modules
- Kafka/event streaming unless forced
- graph/geo/analytics implementation before core CRUD/search works
- k8s/Terraform/Helm-heavy investment before one deployable app exists
- generic shared packages without a real consumer need

## Immediate Next Steps In This Repo

If I were driving this repo, the first concrete tasks would be:

1. Clean naming and identity across the repo.
2. Replace placeholder docs with a real domain model and first-slice architecture.
3. Make docker-compose.yml run Postgres, gateway, web, and entity-service.
4. Turn entity-service into the first real backend.
5. Turn gateway into the single public API.
6. Build the first usable UI around search + detail.
