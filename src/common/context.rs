use tokio::sync::OnceCell;
use crate::common::{
    config::Config,
    database::Connection,
    errors::Error,
};

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
        self.connection.get_or_try_init(|| async {
            Connection::new(&self.config.database).await
        }).await
    }
}