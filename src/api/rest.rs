use actix_web::{App, HttpResponse, HttpServer, Responder, get, patch, post, web};

#[get("/languages/list")]
async fn languages_list() -> impl Responder {
    HttpResponse::Ok().body("Available languages: en, es, fr, de")
}

#[post("/languages/add/{name}")]
async fn languages_add() -> impl Responder {
    HttpResponse::Ok().body(format!("Language {} added!!", "dd"))
}

#[patch("/languages/update/{name}")]
async fn languages_update() -> impl Responder {
    HttpResponse::Ok().body(format!("Language {} updated!!", "dd"))
}

/**
 * Starts the REST API server on the specified port.
 * The server listens on all interfaces and serves the defined endpoints.
 */
pub fn start_server(port: u16) {
    let address = format!("0.0.0.0:{}", port);
    let server = HttpServer::new(|| {
        App::new()
            .service(languages_list)
            .service(languages_add)
            .service(languages_update)
    })
    .bind(address)
    .expect("Failed to bind server")
    .run();

    let _ = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(server);
}