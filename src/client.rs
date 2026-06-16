use actix_web::{HttpResponse, Responder, get};

pub mod videos;

#[get("/{tail:.*}")]
async fn app_shell() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(videos::spa_html())
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(app_shell);
}
