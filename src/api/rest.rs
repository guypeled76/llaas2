use actix_web::{App, HttpResponse, HttpServer, Responder, get, patch, post, web::Json};
use validator::Validate;

use crate::api::types::Language;




#[get("/languages/list")]
async fn languages_list() -> impl Responder {
    HttpResponse::Ok().body("Available languages: en, es, fr, de")
}

#[post("/languages/add")]
async fn languages_add(body: Json<Language>) -> impl Responder {
    match body.validate() {
        Err(errors) => HttpResponse::BadRequest().body(format!("Validation errors: {:?}", errors)),
        Ok(_) => HttpResponse::Ok().body(format!("Language {} added!!", body.name)),
    }
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