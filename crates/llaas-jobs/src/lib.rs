//! Job payloads and worker orchestration for LLAAS.
//!
//! Apalis task payloads, enqueueing, and job handlers belong here. The backend
//! decides how workers are started.

use apalis::prelude::{BoxDynError, WorkerContext};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoopJob {
    pub id: String,
}

pub async fn handle_noop_job(_job: NoopJob, _worker: WorkerContext) -> Result<(), BoxDynError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::NoopJob;
    use apalis::prelude::*;
    use apalis_sqlite::{SqlitePool, SqliteStorage};

    #[tokio::test]
    async fn sqlite_storage_accepts_noop_job() {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        SqliteStorage::setup(&pool).await.unwrap();

        let mut storage = SqliteStorage::new(&pool);
        storage
            .push(NoopJob {
                id: "noop".to_string(),
            })
            .await
            .unwrap();
    }
}
