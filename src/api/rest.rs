use actix_web::{App, HttpResponse, HttpServer, Responder, get, patch, post, web::{Json, Path}};
use validator::Validate;

use crate::api::types::{LanguageRequest, LanguageUrl};




#[get("/languages/list")]
async fn languages_list() -> impl Responder {
    HttpResponse::Ok().body("Available languages: en, es, fr, de")
}

#[post("/languages/add")]
async fn languages_add(body: Json<LanguageRequest>) -> impl Responder {
    match body.validate() {
        Err(errors) => HttpResponse::BadRequest().body(format!("Validation errors: {:?}", errors)),
        Ok(_) => HttpResponse::Ok().body(format!("Language {} added!!", body.name)),
    }
}

#[patch("/languages/update/{code}")]
async fn languages_update(url: Path<LanguageUrl>) -> impl Responder {
    match url.validate() {
        Err(errors) => HttpResponse::BadRequest().body(format!("Validation errors: {:?}", errors)),
        Ok(_) => HttpResponse::Ok().body(format!("Language {} updated!!", url.code)),
    }
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