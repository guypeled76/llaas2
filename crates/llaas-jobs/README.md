# llaas-jobs

Apalis job payloads, enqueueing, and job handlers for LLAAS.

Queue internals should stay hidden behind service APIs; client-visible job state is mirrored through the service layer.

## Verification

```sh
cargo check -p llaas-jobs
```