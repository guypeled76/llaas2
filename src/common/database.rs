use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::auth::Root;

use crate::common::errors::Error;

// A module to handle database connections and operations using SurrealDB with a RocksDB engine. 
// It provides a Connection struct that initializes the database connection and allows for future 
// expansion to include methods for querying and manipulating the database as needed.
pub struct Connection {
    db: Surreal<Db>,
}

impl Connection {

    // Initialize the database connection
    pub async fn new() -> Result<Self, Error> {
        // Create a connection that uses /resources/db as the storage path.
        let db = Surreal::new::<RocksDb>("/resources/db").await?;
        db.signin(Root { 
            username: "root".into(), 
            password: "root".into() 
        })
        .await?;
        db.use_ns("llaas").use_db("main").await?;
        Ok(Self { db })
    }
}