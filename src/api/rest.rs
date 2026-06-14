use actix_web::{
    App, HttpResponse, HttpRequest, HttpServer, Responder, 
    get, patch, post, 
    web::{self, Json, Path}
};

use validator::Validate;

use crate::api::types::{LanguageRequest, LanguageUrl};
use crate::resources::video;

struct AppConfig {
    port: u16,
}


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

#[patch("/videos/{id}/{lang}/subtitles.vtt")]
async fn video_vtt(path: Path<(uuid::Uuid, String)>) -> impl Responder {
    let (id, lang) = path.into_inner();
    match video::subtitles(id, &lang) {
        Ok(subtitle) => HttpResponse::Ok().body(subtitle),
        Err(_) => HttpResponse::NotFound().body(format!("Subtitle for video {} in language {} not found!!", id, lang)),
    }
}

#[patch("/videos/{id}/{lang}/view.html")]
async fn video_view(path: Path<(uuid::Uuid, String)>) -> impl Responder {
    let (id, lang) = path.into_inner();
    match video::view(id, &lang) {
        Ok(view) => HttpResponse::Ok().body(view),
        Err(_) => HttpResponse::NotFound().body(format!("View for video {} in language {} not found!!", id, lang)),
    }
}

#[patch("/videos/{id}/stream")]
async fn video_stream(req: HttpRequest, path: Path<uuid::Uuid>) -> impl Responder {
    let id = path.into_inner();
    video::stream(req, id)
}

/**
 * Starts the REST API server on the specified port.
 * The server listens on all interfaces and serves the defined endpoints.
 */
pub fn start_server(port: u16) {
    
    // Create an instance of AppConfig so we can use it from the services. 
    let config = web::Data::new(AppConfig { port });

    // Construct the address to bind the server to, using the specified port. The server will listen on all interfaces (
    let address = format!("0.0.0.0:{}", port);

    // Create and run the Actix-web server with the defined routes and handlers and set the AppState with the specified port. The server will run indefinitely until it is stopped.
    let server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(languages_list)
            .service(languages_add)
            .service(languages_update)
            .service(video_vtt)
            .service(video_view)
            .service(video_stream)
    })
    .bind(address)
    .expect("Failed to bind server")
    .run();

    let _ = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(server);
}