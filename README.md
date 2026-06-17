# LLAAS

LLAAS is a language learning platform server. The current implementation is still the original single `llaas` package, while the repository is being prepared for a workspace split.

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

The root `llaas` package remains the behavior-preserving implementation until code is moved into the workspace crates.

```sh
cargo check --workspace
```