use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::auth::Root;
use surrealdb_types::RecordId;

use std::path::Path;

use crate::common::{
    config::DatabaseConfig, 
    errors::Error
};

// A module to handle database connections and operations using SurrealDB with a RocksDB engine. 
// It provides a Connection struct that initializes the database connection and allows for future 
// expansion to include methods for querying and manipulating the database as needed.
pub struct Connection {
    db: Surreal<Db>,
}

impl Connection {

    // Initialize the database connection
pub async fn new(config: &DatabaseConfig) -> Result<Self, Error> {

        // Check if the database file already exists to determine if we need to initialize it with credentials.
        let is_new_db = !Path::new(&config.path).exists();

        // Create a connection that uses the specified path from the configuration.
        let db = Surreal::new::<RocksDb>(&config.path).await?;

        // If the database is new, initialize it with dynamic credentials from the configuration.
        if is_new_db {

            println!("Initializing empty database with dynamic credentials...");
        
            // Pass variables into the query using the '$' prefix
            let query = format!(
                "DEFINE USER {} ON ROOT PASSWORD '{}' ROLES OWNER;",
                config.username, 
                config.password
            );
            db.query(&query).await?;

            // Create namespace and database if they don't exist
            db.query("DEFINE NAMESPACE llaas; USE NS llaas; DEFINE DATABASE main;").await?;

        }

        println!("Setting namespace and database for future operations...");

        // Switch to the appropriate namespace and database for future operations.
        db.use_ns("llaas").use_db("main").await?;

        println!("Signing in to the database with provided credentials...");

        // Sign in to the database using the credentials from the configuration.
        db.signin(Root { 
            username: config.username.clone(), 
            password: config.password.clone() 
        })
        .await?;

    
        Ok(Self { db })
    }

    /// Get a reference to the SurrealDB instance for performing database operations.
    pub fn db(&self) -> &Surreal<Db> {
        &self.db
    }
}


/// A convenience method to create a RecordId from a table name and key.
pub fn record(table: &str, key: &str) -> RecordId {
    RecordId::new(table, key)
}