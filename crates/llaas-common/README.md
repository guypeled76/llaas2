# llaas-common

Shared native utilities for the workspace: configuration, errors, tracing, path settings, shutdown helpers, and small utility traits.

This crate may depend on `llaas-core`, but should not depend on backend, frontend, store, jobs, resources, or AI crates.

## Verification

```sh
cargo check -p llaas-common
```