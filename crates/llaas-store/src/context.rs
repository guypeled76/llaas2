use surrealdb::Surreal;
use surrealdb::engine::local::Db;

use crate::database::Connection;
use llaas_common::{config::Config, errors::Error};
use tokio::sync::OnceCell;

pub struct Context {
    connection: OnceCell<Connection>,
    config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            connection: OnceCell::new(),
            config,
        }
    }

    pub async fn connection(&self) -> Result<&Connection, Error> {
        self.connection
            .get_or_try_init(|| async {
                println!("Initializing database connection...");
                Connection::new(&self.config.database).await
            })
            .await
    }

    pub async fn db(&self) -> Result<&Surreal<Db>, Error> {
        Ok(self.connection().await?.db())
    }
}
