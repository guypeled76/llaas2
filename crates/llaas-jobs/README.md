# llaas-jobs

Apalis job payloads, enqueueing, and job handlers for LLAAS.

Queue internals should stay hidden behind service APIs; client-visible job state is mirrored through the service layer.

## Apalis Versioning

The Apalis 1.0 release-candidate crates are pinned as a compatible family. Do not loosen `apalis`, `apalis-core`, `apalis-codec`, `apalis-sql`, or `apalis-sqlite` independently without rerunning the SQLite enqueue smoke test; Cargo can otherwise select mismatched RC versions with incompatible trait shapes.

## Verification

```sh
cargo check -p llaas-jobs
cargo test -p llaas-jobs sqlite_storage_accepts_noop_job
```