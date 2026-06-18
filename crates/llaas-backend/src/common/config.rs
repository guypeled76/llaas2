use std::env;

pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            database: DatabaseConfig {
                path: env::var("DATABASE_PATH").unwrap_or_else(|_| String::from("./resources/db")),
                username: env::var("DATABASE_USERNAME").unwrap_or_else(|_| String::from("root")),
                password: env::var("DATABASE_PASSWORD").unwrap_or_else(|_| String::from("root")),
            },
        }
    }
}

pub struct DatabaseConfig {
    pub path: String,
    pub username: String,
    pub password: String,
}
