use actix_web::{HttpRequest, HttpResponse, Responder, get};

pub mod app;
pub mod homepage;
pub mod videos;

use futures::StreamExt;
use leptos::prelude::*;
use leptos_router::location::RequestUrl;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(app_shell);
}

#[get("/{tail:.*}")]
async fn app_shell(req: HttpRequest) -> impl Responder {
    let owner = Owner::new();
    let view = owner.with(|| {
        provide_context(RequestUrl::new(req.uri().path()));
        view! {
            <html>
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <title>LLAAS</title>
                    <style>
                        {r#"
                            body { font-family: sans-serif; margin: 40px; background: #121212; color: #fff; }
                            #subtitle-timeline { margin-top: 20px; max-height: 300px; overflow-y: auto; padding: 10px; background: #1e1e1e; border-radius: 6px; }
                            button { margin: 6px 0; padding: 10px; cursor: pointer; display: block; width: 100%; text-align: left; background: #2a2a2a; color: #fff; border: 1px solid #3a3a3a; border-radius: 4px; }
                            button:hover { background: #3a3a3a; }
                            video { border-radius: 6px; background: #000; max-width: 100%; height: auto; }
                            input { margin-right: 8px; margin-bottom: 8px; padding: 8px; }
                        "#}
                    </style>
                </head>
                <body>
                    <div id="app">
                        <app::App/>
                    </div>
                </body>
            </html>
        }
    });

    // SSR-only (no hydration): use the in-order HTML stream, which awaits each
    // <Suspense> before emitting, and collect it into a fully-resolved string.
    // ScopedFuture keeps the reactive owner/observer active while polling.
    let html = owner
        .with(|| ScopedFuture::new(view.to_html_stream_in_order().collect::<String>()))
        .await;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
