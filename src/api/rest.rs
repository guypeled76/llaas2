use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/test")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

/**
 * Starts the REST API server on the specified port.
 * The server listens on all interfaces and serves the defined endpoints.
 */
pub fn start_server(port: u16) {
    let address = format!("0.0.0.0:{}", port);
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(address)
    .expect("Failed to bind server")
    .run();

    let _ = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(server);
}