# llaas-frontend

Leptos CSR/hydrated server/admin UI for LLAAS.

This crate should stay WASM-safe and must not depend directly on backend, store, jobs, or AI implementation crates.

## Verification

```sh
cargo check -p llaas-frontend
```