use actix_web::{HttpResponse, Responder, get, web};
use leptos::prelude::*;

const HELLO_TITLE: &str = "Hello from Leptos (test 1)";
const HELLO_MESSAGE: &str = "LLAAS UI is running - aaa.";

#[component]
fn HelloPage() -> impl IntoView {
    view! {
        <main>
            <h1>{HELLO_TITLE}</h1>
            <p>{HELLO_MESSAGE}</p>
        </main>
    }
}

#[get("/")]
async fn hello_world() -> impl Responder {
    let _hello_page_component = HelloPage;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>LLAAS</title></head><body><main><h1>{}</h1><p>{}</p></main></body></html>",
            HELLO_TITLE, HELLO_MESSAGE
        ))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world);
}
