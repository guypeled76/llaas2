# LLAAS

LLAAS is a language learning platform server. The repository is being split into workspace crates while preserving the existing command behavior.

The refactor plan lives in [plans/workspace-graphql-refactor.md](plans/workspace-graphql-refactor.md).

## Workspace Crates

- `llaas-core` - pure domain types.
- `llaas-common` - shared native configuration, errors, and utilities.
- `llaas-api` - public GraphQL and REST contract types.
- `llaas-store` - SurrealDB repositories.
- `llaas-ai` - AI and TTS model integrations.
- `llaas-resources` - EPUB, video, subtitle, and media processing.
- `llaas-jobs` - Apalis job payloads and handlers.
- `llaas-backend` - Actix server, GraphQL, REST/media, UI serving, and workers.
- `llaas-cli` - command-line client.
- `llaas-frontend` - Leptos server/admin UI.

## Current Checkpoint

The root `llaas` package is now a thin compatibility binary that delegates to `llaas-cli`. The current implementation has been moved into workspace crates, while later phases will replace local CLI calls with GraphQL service calls.

```sh
cargo check --workspace
```