use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, get, patch, post,
    web::{self, Json, Path},
};

use validator::Validate;

use crate::api::types::{LanguageRequest, LanguageUrl};
use crate::common::{context::Context, errors::Error};
use crate::resources::video;

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

#[get("/videos/{id}/{lang}/subtitles.vtt")]
async fn video_vtt(
    context: web::Data<&'static Context>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (id, lang) = path.into_inner();
    match video::subtitles(&context, &id, &lang) {
        Ok(subtitle) => HttpResponse::Ok().body(subtitle),
        Err(_) => HttpResponse::NotFound().body(format!(
            "Subtitle for video {} in language {} not found!!",
            id, lang
        )),
    }
}

#[get("/videos/{id}/{lang}/view.html")]
async fn video_view(
    context: web::Data<&'static Context>,
    path: Path<(String, String)>,
) -> impl Responder {
    let (id, lang) = path.into_inner();
    match video::view(&context, &id, &lang) {
        Ok(view) => HttpResponse::Ok().body(view),
        Err(_) => HttpResponse::NotFound().body(format!(
            "View for video {} in language {} not found!!",
            id, lang
        )),
    }
}

#[get("/videos/{id}.mp4")]
async fn video_stream(
    context: web::Data<&'static Context>,
    req: HttpRequest,
    path: Path<String>,
) -> impl Responder {
    let id = path.into_inner().clone();
    video::stream(&context, req, id)
}

/**
 * Starts the REST API server on the specified port.
 * The server listens on all interfaces and serves the defined endpoints.
 */
pub fn start_server(context: &'static Context, port: u16) {
    // Construct the address to bind the server to, using the specified port. The server will listen on all interfaces (
    let address = format!("0.0.0.0:{}", port);

    // Create and run the Actix-web server with the defined routes and handlers and set the AppState with the specified port. The server will run indefinitely until it is stopped.
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context))
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

    let _ = tokio::runtime::Runtime::new().unwrap().block_on(server);
}
