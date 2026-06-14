use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;


// A module to handle database connections and operations using SurrealDB with a RocksDB engine. 
// It provides a Connection struct that initializes the database connection and allows for future 
// expansion to include methods for querying and manipulating the database as needed.
pub struct Connection {
    db: Surreal<RocksDb>,
}

impl Connection {

    // Initialize the database connection
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let db = Surreal::new::<RocksDb>("./resources/data").await?;
        db.signin(surrealdb::auth::Root {
            username: "root",
            password: "root",
        })
        .await?;
        db.use_ns("llaas").use_db("main").await?;
        Ok(Self { db })
    }
}