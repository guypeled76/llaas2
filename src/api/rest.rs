use actix_web::{
    HttpRequest, HttpResponse, Responder, get, patch, post,
    web::{self, Json, Path},
};

use validator::Validate;

use crate::api::types::{LanguageRequest, LanguageUrl};
use crate::common::context::Context;
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

#[get("/videos/{id}.mp4")]
async fn video_stream(
    context: web::Data<&'static Context>,
    req: HttpRequest,
    path: Path<String>,
) -> impl Responder {
    let id = path.into_inner().clone();
    video::stream(&context, req, id)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(languages_list)
        .service(languages_add)
        .service(languages_update)
        .service(video_vtt)
        .service(video_stream);
}
