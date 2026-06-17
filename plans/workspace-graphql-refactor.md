## Plan: Split LLAAS Workspace Around GraphQL

Target repository copy: `/Users/guy.peled/Development/github/llaas/plans/workspace-graphql-refactor.md`. The implementation handoff should create the root `plans/` directory if it does not exist and copy this plan there as the canonical repo-visible planning document.

Refactor the current single `llaas` binary into a Rust workspace that preserves existing behavior first, then introduces the target service architecture: remote SurrealDB as the domain database, Apalis + SQLite for durable long-running jobs, GraphQL as the primary client API, REST only for media/operational endpoints that GraphQL is bad at, a CSR/hydrated Leptos frontend, and a CLI that talks to the same GraphQL service as external clients.

GraphQL replaces the earlier gRPC/gRPC-Web plan. This is a better fit for client adoption because clients can use plain HTTP JSON, introspection, generated TypeScript/Rust clients, and flexible selection sets without protobuf/gRPC-Web tooling. The tradeoff is that the backend must actively control schema stability, query depth/complexity, pagination, authorization, and N+1 query behavior.

## Recommended Server and UI Stack

- Use one Actix Web server in `llaas-backend` for all backend surfaces: GraphQL, GraphQL subscriptions, REST/media endpoints, health/readiness/metrics, Apalis board API routes, and the built Leptos admin frontend.
- Use `async-graphql` for a code-first GraphQL schema and resolvers. It supports async/await, Actix integration, custom scalars, multipart uploads, subscriptions over WebSocket, GraphiQL/Playground-style IDEs, DataLoader, tracing/extensions, and query depth/complexity controls.
- Use `async-graphql-actix-web` for `/graphql` HTTP queries/mutations and `/graphql/ws` subscriptions.
- Expose a GraphQL explorer inside the server UI. Recommendation: serve GraphiQL v2 from `async_graphql::http::GraphiQLSource` behind the frontend route `/admin/graphql`, connected to `/graphql` and `/graphql/ws`. Gate production access behind auth/config.
- Use `apalis-board` for job UI. It provides Actix API utilities and a Leptos web UI, supports `apalis-sqlite`, and can be mounted/embedded in the admin frontend. Recommendation: mount board APIs under `/admin/jobs/api/*` and expose its UI under `/admin/jobs` as part of the same frontend navigation.
- Keep REST/media endpoints for video range streaming, subtitle VTT files, health checks, metrics, and any large upload/download path that should not go through GraphQL JSON payloads.
- For Rust clients, use `cynic` or `graphql_client` after the schema stabilizes. For the Leptos frontend, start with a small HTTP GraphQL client wrapper, then move to typed generated queries once the schema settles.

## Target Workspace Crates

1. `crates/llaas-core` — pure domain types and value objects: language codes, resource IDs, resources, signals, client-visible job status types, subtitle cue/domain book structures. No Actix, Leptos, SurrealDB, Apalis, async-graphql, rust-bert, tch, or filesystem logic.
2. `crates/llaas-common` — cross-cutting native utilities: configuration, error types, tracing/logging setup, path configuration, shutdown helpers, and shared utility traits. Keep it intentionally small.
3. `crates/llaas-api` — public API contract crate. Owns code-first GraphQL schema types, custom scalars, input/output DTOs, SDL export, checked/persisted operation documents, REST DTOs for public non-GraphQL endpoints, and conversions to/from `llaas-core`.
4. `crates/llaas-store` — remote SurrealDB connection and repositories. This is where `surrealdb` belongs, not `llaas-core` or `llaas-common`.
5. `crates/llaas-ai` — heavy AI/ML wrappers for keywords, translation, classification, sentiment, POS, and TTS. This isolates `rust-bert`, `tch`, and `any-tts` from clients and most API code.
6. `crates/llaas-resources` — EPUB parsing, video download/discovery, subtitle parsing, media filesystem layout, and pure resource processing. It should not own HTTP handlers or database connections.
7. `crates/llaas-jobs` — Apalis task payloads, job enqueue API, and job handlers/workflow orchestration. It can depend on store/resources/AI, while backend decides how to run it.
8. `crates/llaas-backend` — the single service/server process. Owns Actix runtime bootstrap, GraphQL route wiring, REST/media routes, GraphQL explorer route, Apalis board API mounting, static frontend serving, SurrealDB repository wiring, and Apalis worker monitor startup in the same process.
9. `crates/llaas-cli` — CLI binary. During the behavior-preserving phase it may call local libraries; final target is a GraphQL client that submits jobs, resources, and admin commands to the service.
10. `crates/llaas-frontend` — Leptos CSR/hydrated WASM server/admin UI. Talks to the backend through GraphQL queries/mutations/subscriptions, uses REST/media URLs for video/subtitle assets, and contains navigation/pages for the main app UI, GraphQL explorer, and Apalis board.

Intended dependency direction:

- `llaas-core` has no workspace dependencies.
- `llaas-common` may depend on `llaas-core`, but not on backend/frontend/store/jobs/AI.
- `llaas-api` depends on `llaas-core`, `llaas-common` if needed, and `async-graphql` for schema/input/output types. It should not depend on store/jobs/resources/AI.
- `llaas-store` depends on `llaas-core`, `llaas-common`, and `surrealdb` remote protocol features.
- `llaas-ai` depends on `llaas-common` and heavy model crates.
- `llaas-resources` depends on `llaas-core`, `llaas-common`, and optionally `llaas-ai` through explicit traits/features.
- `llaas-jobs` depends on `llaas-core`, `llaas-common`, `llaas-store`, `llaas-resources`, and `llaas-ai`.
- `llaas-backend` depends on `llaas-api`, store, jobs, resources, and frontend/static serving support, and owns resolver wiring to application services.
- `llaas-cli` should depend on a GraphQL client stack and schema/operation documents, not backend internals in the final state.
- `llaas-frontend` should depend only on WASM-safe client code, operation documents, Leptos, and Apalis board web components/assets where feasible; it must not depend on backend/store/jobs/AI.

## Steps

### Phase 0: Baseline and Safety Rails

1. Capture the current working baseline before moving files: run `cargo check`, `cargo test` if tests exist, and a smoke run of current CLI commands that are practical locally (`book`, `tts` if model setup permits, `start`, and `video` only if `yt-dlp`/network is available).
2. Record known current-state risks: no existing READMEs, no job queue, no GraphQL schema, embedded SurrealDB only, SSR-only frontend, blocking video/model work, protobuf messages with no services.
3. Use `llaas-api` as the public contract crate name. It owns GraphQL schema types and public REST DTOs because GraphQL and REST come from the same server.

Verification:
- `cargo check` result recorded before refactor.
- Current command behavior noted, including any model/download prerequisites that cannot run locally.

### Phase 1: Create Workspace Without Behavior Changes

4. Convert root `Cargo.toml` to a workspace manifest with `resolver = "3"`, shared `[workspace.package]`, and shared `[workspace.dependencies]`. Keep dependency versions aligned with the current lockfile where possible.
5. Create initial crates with empty libraries/binaries and crate-level READMEs: `llaas-core`, `llaas-common`, `llaas-api`, `llaas-store`, `llaas-ai`, `llaas-resources`, `llaas-jobs`, `llaas-backend`, `llaas-cli`, and `llaas-frontend`.
6. Move current protobuf-generated content messages from root `build.rs` and `src/messages/*` into a temporary compatibility module only if needed by current EPUB JSON behavior. Do not expand protobuf into service definitions.
7. Move `src/common/config.rs` and `src/common/errors.rs` into `llaas-common`. Keep error conversions temporarily broad enough for current code, then narrow after store/AI boundaries settle.
8. Move `src/common/database.rs`, `src/common/context.rs`, and `src/database/videos.rs` into `llaas-store` as transitional embedded SurrealDB implementation. This preserves behavior before switching to remote SurrealDB.
9. Move `src/models/*` into `llaas-ai` unchanged except for import paths.
10. Move `src/resources/epub.rs` and `src/resources/video.rs` into `llaas-resources` unchanged except for imports. Keep the current direct database upsert for this phase if needed to keep behavior compiling.
11. Move `src/api/*` and existing Actix server bootstrap into `llaas-backend`.
12. Move CLI command parsing and `book_to_json` from `src/main.rs` into `llaas-cli`, calling migrated libraries directly for now.
13. Keep the current Leptos SSR code either inside `llaas-backend` temporarily or in a transitional feature of `llaas-frontend`; do not rewrite it to CSR until the GraphQL schema exists.

Verification:
- `cargo check --workspace`.
- `cargo check -p llaas-cli` and `cargo check -p llaas-backend`.
- Existing CLI commands still compile and route to the same implementations.
- Existing REST/video routes still compile.

### Phase 2: Clean Crate Boundaries

14. Define pure `llaas-core` domain types for `Video`, `Language`, resource IDs, job IDs/statuses, signal/message/resource concepts, subtitle cue/timeline, and book/article structures.
15. Move SurrealDB-specific `RecordId` and `SurrealValue` out of domain structs. Implement conversions in `llaas-store` instead of leaking database types into `llaas-core`.
16. Replace the static leaked `Context` pattern with explicit `Arc<AppState>` in `llaas-backend`; `AppState` should hold config, repositories, job enqueue handle, media root, and model/job services as needed.
17. Decouple `llaas-resources::video::download` from `Context` and `VideoDatabase`. It should return a domain/resource result; persistence should happen in service/job handlers.
18. Decouple `llaas-resources::epub::read` from direct `KeywordsModel::apply`. Introduce a trait or optional step so EPUB parsing can run without forcing model loading.
19. Replace hardcoded `resources/videos` with configured `media_root`, and store relative media paths in SurrealDB.
20. Replace broad `unwrap`/`expect` in service-facing code paths with crate errors while allowing examples/demo commands to stay simpler until converted.

Verification:
- `cargo check --workspace`.
- Unit tests for pure transformations: EPUB chapter parsing, subtitle cue conversion, media path construction, domain/GraphQL DTO conversions.
- No `surrealdb`, `actix-web`, `leptos`, `apalis`, `async-graphql`, `rust-bert`, or `tch` dependency in `llaas-core`.
- No `actix-web` dependency in `llaas-resources` after media streaming moves to backend.

### Phase 3: Remote SurrealDB Store

21. Replace embedded RocksDB connection in `llaas-store` with remote SurrealDB over WebSocket/WSS. Config should include `SURREALDB_URL`, `SURREALDB_NS`, `SURREALDB_DB`, `SURREALDB_USERNAME`, and `SURREALDB_PASSWORD`.
22. Remove `kv-rocksdb` from the main service path. Keep optional dev/test support only if useful, but the target is remote server only per user decision.
23. Add repository traits and implementations for videos/resources/jobs/signals/languages. Clients should read job/resource status from GraphQL-backed application services, not from Apalis internals.
24. Add schema/bootstrap statements or migration files for SurrealDB tables/indexes/permissions. Do not leave schema as implicit upserts only.
25. Add an optional migration utility or documented manual path for existing `resources/db` data if preserving local data matters.

Verification:
- `cargo check -p llaas-store` with remote protocol features.
- Integration test can connect to a test SurrealDB instance, bootstrap schema, upsert/select a video/resource/job status, and tear down test data.
- Backend fails fast with a clear error if SurrealDB config is missing or unreachable.

### Phase 4: GraphQL Schema and REST Media Gateway

26. Define the GraphQL schema code-first in `llaas-api` with `async-graphql`: derive `SimpleObject`, `InputObject`, `Enum`, `Union`, and custom scalar/newtype mappings from domain DTOs, and define `QueryRoot`, `MutationRoot`, and `SubscriptionRoot` with `#[Object]`/`#[Subscription]` impl blocks.
27. Keep resolver business logic out of `llaas-api`. The API crate defines schema-facing types and resolver traits/adapters; `llaas-backend` injects `Arc<AppState>` and wires resolvers to application services, repositories, and job enqueue handles.
28. Start with schema areas that map directly to client needs:
    - `Query.languages`, `Query.resources`, `Query.resource(id)`, `Query.jobs`, `Query.job(id)`, `Query.serverInfo`.
    - `Mutation.submitVideo`, `Mutation.submitEpub`, `Mutation.submitSignal`, `Mutation.requestTts`, `Mutation.updateLanguage`.
    - `Subscription.jobUpdated(jobId)`, `Subscription.resourceUpdated(resourceId)` where useful, always with polling query fallback.
29. Use GraphQL custom scalars/newtypes for domain IDs and language codes. Keep resolver return shapes client-oriented, not database-row-oriented.
30. Add cursor or page-token pagination to list fields from the beginning. Do not expose unbounded resource/job/signal lists.
31. Add DataLoader or batched repository methods for nested fields that could cause N+1 SurrealDB queries, especially resource -> jobs/signals/media/subtitles.
32. Add GraphQL depth/complexity limits, request body limits, auth hooks/guards, structured error extensions, and optional persisted queries once clients stabilize.
33. Export SDL from the code-first schema with `schema.sdl()` for documentation, schema checks, and client generation. Treat the exported SDL as a generated artifact or checked-in compatibility contract.
34. Expose GraphiQL v2 as the embedded documentation/query UI. Backend serves the actual GraphiQL HTML; frontend routes to it under `/admin/graphql` so users experience it as part of the server UI.
35. Keep media bytes out of GraphQL where they are large or range-based. Serve video and subtitle files through REST/media endpoints such as `/media/videos/{id}.mp4` and `/media/videos/{id}/{lang}/subtitles.vtt`.
36. Implement REST endpoints in the same Actix server only where GraphQL is not appropriate: health/readiness, metrics, static frontend assets, MP4 range streaming, VTT files, large uploads if needed, and optional webhook-style callbacks.

Verification:
- `cargo check -p llaas-api -p llaas-backend` or equivalent package names.
- GraphQL SDL export is committed or generated as an artifact for client tooling.
- GraphQL smoke test can call `languages` and `submitVideo` against backend.
- Subscription or polling smoke test can observe a job status update.
- REST media range request returns expected partial content for a video fixture.

### Phase 5: Apalis SQLite Jobs in Backend Process

37. Run a small dependency spike in `llaas-jobs` before full integration: compile a no-op Apalis SQLite job and worker using the chosen version pair. Prefer stable Apalis if compatible; otherwise pin a known working RC pair.
38. Add job payload types for the initial pipelines:
    - `DownloadVideo { resource_id, url, languages }`
    - `ParseSubtitles { resource_id, media_paths }`
    - `IngestEpub { resource_id, source }`
    - `ExtractKeywords { resource_id, text_refs }`
    - `TranslateText`, `ClassifyText`, `AnalyzeSentiment`, `TagPartsOfSpeech`, `SynthesizeSpeech` as later AI jobs.
39. Store operational queue state in SQLite via Apalis, but mirror user-visible job summaries/status/progress/results in SurrealDB. GraphQL resolvers should read the SurrealDB-visible projection.
40. Mount `apalis-board` API routes in the same Actix server, scoped under `/admin/jobs/api/*`, registering the SQLite job stores used by LLAAS workers.
41. Expose Apalis job UI through `llaas-frontend` under `/admin/jobs`. Prefer embedding `apalis-board` Leptos web components/assets via its `web` feature; fallback is to mount `ServeApp` under the same admin route and link/iframe it from the frontend shell.
42. Start with explicit follow-up job enqueueing instead of relying on Apalis workflow/DAG APIs. Introduce `apalis-workflow` only after the core queue is stable.
43. Wrap blocking operations (`yt-dlp`, rust-bert/tch model inference, TTS synthesis, synchronous filesystem scans) in controlled blocking execution or dedicated worker concurrency limits.
44. Start API and workers in the same backend process by default, per user decision. Keep worker startup modular so a future worker-only binary remains easy.

Verification:
- `cargo check -p llaas-jobs -p llaas-backend`.
- Integration test enqueues a no-op job, runs a worker, marks it complete, mirrors status to SurrealDB, and exposes status through GraphQL.
- Video download job can be tested with a mocked downloader trait before requiring `yt-dlp`/network.
- Failed jobs retry according to configured attempts and produce readable GraphQL-visible status/errors.

### Phase 6: CLI as a GraphQL Client

45. Convert `llaas-cli` from direct library calls to GraphQL HTTP calls for normal operations: submit video, submit/import EPUB, synthesize TTS, inspect jobs, list resources/languages, and administer schema/bootstrap if needed.
46. Choose a client strategy after schema stabilizes:
    - Start simple with `reqwest` and checked operation documents.
    - Move to `cynic` or `graphql_client` for typed Rust query/mutation structs.
47. Keep intentionally local/developer-only commands behind names like `local-*` or features if direct processing remains valuable.
48. Add output modes such as human-readable table/plain text and JSON for scripting.

Verification:
- CLI can call a running backend GraphQL endpoint and submit a job.
- CLI can poll or subscribe to job status.
- CLI exits non-zero on GraphQL errors and prints useful diagnostics from error extensions.

### Phase 7: Leptos CSR/Hydrated Frontend over GraphQL

49. Convert `llaas-frontend` into a CSR/hydrated Leptos app built for `wasm32-unknown-unknown`, likely with Trunk or the chosen Leptos build tool.
50. Treat `llaas-frontend` as the server/admin UI shell. It should own navigation and layout for the main LLAAS console, resource/job views, GraphQL explorer, and Apalis board.
51. Remove Leptos server functions as primary data loading. Replace them with GraphQL queries/mutations/subscriptions over HTTP/WebSocket.
52. Start with a small GraphQL HTTP client wrapper in the frontend while schema is moving. Once stable, generate typed Rust or TypeScript query bindings from the exported SDL and checked operation documents.
53. Add frontend routes:
    - `/admin` for overview/server dashboard.
    - `/admin/resources`, `/admin/jobs`, `/admin/videos`, `/admin/languages` for native LLAAS views.
    - `/admin/graphql` for embedded GraphiQL/documentation/query UI.
    - `/admin/apalis` or `/admin/jobs/board` for the Apalis board if it cannot be cleanly mounted directly at `/admin/jobs`.
54. Build initial frontend flows around the service contract: submit resource/signal, list resources, inspect jobs, open video, view subtitle timeline, and call media REST endpoints for actual video/subtitle assets.
55. Keep frontend dependencies WASM-safe: no `surrealdb` server engine, no `tokio` multi-thread server transport, no `rust-bert/tch`, no Actix, and no direct Apalis worker/storage dependency. Apalis board web components/assets are allowed if they compile cleanly for the frontend.
56. Backend serves the built frontend static files and handles SPA fallback under `/admin/*`, while `/graphql`, `/graphql/ws`, `/media/*`, `/health`, `/metrics`, and `/admin/jobs/api/*` remain backend routes.

Verification:
- `cargo check -p llaas-frontend --target wasm32-unknown-unknown`.
- `trunk build --release` or equivalent frontend build command succeeds.
- Browser smoke test can call GraphQL `languages`, submit a video job, poll or subscribe to job status, and load media through REST endpoints.

### Phase 8: Documentation and Final Workspace Polish

57. Add root `README.md` explaining the workspace, architecture, crate graph, same-server UI/API model, environment variables, local dev commands, and how to run backend, CLI, frontend, SurrealDB, GraphQL explorer, Apalis board, and Apalis job storage.
58. Add each crate README with purpose, public API, boundaries, dependencies, examples, and verification commands:
    - `crates/llaas-core/README.md`
    - `crates/llaas-common/README.md`
    - `crates/llaas-api/README.md`
    - `crates/llaas-store/README.md`
    - `crates/llaas-ai/README.md`
    - `crates/llaas-resources/README.md`
    - `crates/llaas-jobs/README.md`
    - `crates/llaas-backend/README.md`
    - `crates/llaas-cli/README.md`
    - `crates/llaas-frontend/README.md`
59. Add workspace-level checks: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`, `cargo test --workspace`, frontend WASM build, GraphQL schema export/check, GraphiQL route smoke test, Apalis board route smoke test, and backend/API integration tests.
60. Add a short architectural decision record or README section documenting the key decisions: remote SurrealDB only, Apalis SQLite separate from domain DB, GraphQL primary, REST media/ops only, code-first GraphQL with SDL export, CSR Leptos server/admin frontend, one Actix server for GraphQL/REST/UI/Apalis board, one backend process running API and workers, behavior-preserving split before new architecture.

Verification:
- Expected README files exist.
- Root README can bootstrap a fresh developer through SurrealDB, backend, CLI, frontend, GraphQL endpoint, and media endpoints.
- `cargo check --workspace` and selected integration/frontend checks pass.
- GraphQL SDL export and sample operation documents are available for clients.

## Relevant Existing Files

- `/Users/guy.peled/Development/github/llaas/Cargo.toml` — convert from package manifest to workspace manifest and centralize dependency versions.
- `/Users/guy.peled/Development/github/llaas/build.rs` — protobuf generation becomes temporary compatibility only; no new protobuf service investment in GraphQL plan.
- `/Users/guy.peled/Development/github/llaas/src/main.rs` — split CLI command parsing into `llaas-cli` and backend startup into `llaas-backend`.
- `/Users/guy.peled/Development/github/llaas/src/messages/content.proto` — temporary content-message compatibility source; later replace with core domain types and GraphQL DTOs.
- `/Users/guy.peled/Development/github/llaas/src/common/config.rs` — starting point for common/service config, to expand for SurrealDB remote, GraphQL limits, job/media paths, and frontend/static settings.
- `/Users/guy.peled/Development/github/llaas/src/common/errors.rs` — starting point for shared error handling, later mapped to GraphQL error extensions and HTTP errors.
- `/Users/guy.peled/Development/github/llaas/src/common/database.rs` and `/Users/guy.peled/Development/github/llaas/src/common/context.rs` — current embedded SurrealDB and static context, to become store/app-state boundaries.
- `/Users/guy.peled/Development/github/llaas/src/database/videos.rs` — current Video model/repository, to split into core domain plus store repository.
- `/Users/guy.peled/Development/github/llaas/src/resources/video.rs` — video download, filesystem, subtitles, and HTTP streaming concerns that must be separated.
- `/Users/guy.peled/Development/github/llaas/src/resources/epub.rs` — EPUB parsing coupled to keyword extraction, to decouple for jobs.
- `/Users/guy.peled/Development/github/llaas/src/models/*` — heavy AI/model crate seed.
- `/Users/guy.peled/Development/github/llaas/src/api/rest.rs` and `/Users/guy.peled/Development/github/llaas/src/api/server.rs` — seed for backend Actix runtime, GraphQL endpoint, and REST/media gateway.
- `/Users/guy.peled/Development/github/llaas/src/client.rs` and `/Users/guy.peled/Development/github/llaas/src/client/*` — current SSR frontend to replace with CSR GraphQL frontend.
- `/Users/guy.peled/Development/github/llaas/.cargo/config.toml` — current Apple `tch`/`torch-sys` CXXFLAGS workaround; keep documented in AI crate README.

## Decisions

- Preserve behavior first, then add GraphQL/jobs/frontend architecture.
- Remote SurrealDB only for the target domain database.
- Apalis SQLite is the durable job queue, separate from SurrealDB. SurrealDB stores client-visible job/resource state.
- Backend process runs API and workers together by default.
- One Actix Web server exposes GraphQL, GraphQL subscriptions, REST/media, health/metrics, Apalis board API routes, and the built frontend/admin UI.
- GraphQL is the primary client API. REST remains for media streaming/range requests, uploads if needed, static frontend assets, health, readiness, metrics, and Apalis board support routes.
- GraphQL is defined code-first with `async-graphql`; SDL export is used for documentation, compatibility checks, and client generation.
- Browser frontend uses GraphQL over HTTP/WebSocket, not gRPC-Web.
- `llaas-frontend` is the server/admin UI shell and includes routes for LLAAS views, GraphQL explorer, and Apalis board.
- Large/range media should stay REST/static, not GraphQL JSON.
- CLI should use the same GraphQL API as other clients in the final architecture.
- AI/model crates stay isolated from core/common/frontend/CLI client code.
- Current `Context` should become explicit application state, not a leaked static.

## Challenged Assumptions

- GraphQL is friendlier for clients, but less contract-rigid than protobuf. Mitigate with schema export, checked operation documents, typed client generation, deprecation policy, and schema compatibility tests.
- GraphQL selection flexibility can cause expensive nested queries. Mitigate with pagination, DataLoader, query depth/complexity limits, request limits, and field-level authorization.
- GraphQL subscriptions are useful for job progress, but not every client wants WebSockets. Provide polling queries for every subscription use case.
- GraphQL uploads exist, but video/media flows still fit better as REST upload/download/range endpoints.
- Existing protobuf files are not a strong reason to keep gRPC; they currently define messages only, not services.

## Further Considerations

1. Decide the typed client path after schema stabilizes. Recommendation: start simple with operation documents and HTTP, then adopt `cynic` for Rust CLI/frontend or `graphql_client` if it fits better.
2. Decide how much schema introspection and GraphiQL access to expose in production. Recommendation: enable GraphiQL/introspection in development; gate production introspection behind auth/config until authorization and complexity controls are mature.
3. Validate how deeply `apalis-board` can be embedded in our Leptos admin shell. Recommendation: prefer its `web` feature for native embedding; fallback to mounting its bundled app at an admin subroute while keeping navigation inside `llaas-frontend`.
*** End Patch