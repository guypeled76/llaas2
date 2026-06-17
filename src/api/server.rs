use actix_web::{App, HttpServer, web};

use crate::api::rest;
use crate::client;
use crate::common::context::Context;

pub async fn start_server(context: &'static Context, port: u16) -> std::io::Result<()> {
    let address = format!("0.0.0.0:{}", port);

    // Initialize the global async executor used by Leptos's reactive runtime.
    // Required for server-side rendering of async resources / server functions.
    any_spawner::Executor::init_tokio().expect("Failed to initialize Leptos executor");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context))
            .configure(rest::configure)
            .configure(client::configure)
    })
    .bind(address)?
    .run()
    .await
}
