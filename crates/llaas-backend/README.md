# llaas-backend

Single service/server process for LLAAS.

This crate will own Actix runtime bootstrap, GraphQL route wiring, REST/media routes, GraphQL explorer routes, Apalis board mounting, static frontend serving, SurrealDB repository wiring, and worker startup.

## Verification

```sh
cargo check -p llaas-backend
```