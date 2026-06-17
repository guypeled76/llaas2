# llaas-api

Public API contract crate for LLAAS.

It will own code-first GraphQL schema-facing types, public REST DTOs, SDL export support, operation documents, and conversions to and from `llaas-core` domain types.

Business logic should stay in backend/application services, not in this crate.

## Verification

```sh
cargo check -p llaas-api
```