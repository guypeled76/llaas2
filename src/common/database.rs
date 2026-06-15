use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::auth::Root;

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

        let is_new_db = !Path::new(&config.path).exists();

        // Create a connection that uses the specified path from the configuration.
        let db = Surreal::new::<RocksDb>(&config.path).await?;

        // If the database is new, initialize it with dynamic credentials from the configuration.
        if is_new_db {

            println!("Initializing empty database with dynamic credentials...");
        
            // Pass variables into the query using the '$' prefix
            let query = format!(
                "DEFINE USER {} ON ROOT PASSWORD '{}' ROLES OWNER;",
                config.username, config.password
            );
            db.query(&query).await?;

            // Create namespace and database if they don't exist
            db.query("DEFINE NAMESPACE llaas;").await?;

            // Switch to namespace, then create database
            db.use_ns("llaas").await?;
            db.query("DEFINE DATABASE main;").await?;

        }


        db.signin(Root { 
            username: config.username.clone(), 
            password: config.password.clone() 
        })
        .await?;
        db.use_ns("llaas").await?;
        db.use_db("main").await?;
        Ok(Self { db })
    }

    /// Get a reference to the SurrealDB instance for performing database operations.
    pub fn db(&self) -> &Surreal<Db> {
        &self.db
    }
}