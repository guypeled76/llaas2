use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use crate::api::graphql::LlaasSchema;

#[post("/graphql")]
async fn graphql(schema: web::Data<LlaasSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[get("/graphql/ws")]
async fn graphql_ws(
    schema: web::Data<LlaasSchema>,
    request: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(schema.get_ref().clone()).start(&request, payload)
}

#[get("/admin/graphql")]
async fn graphiql() -> impl Responder {
    let html = GraphiQLSource::build()
        .title("LLAAS GraphQL")
        .endpoint("/graphql")
        .subscription_endpoint("/graphql/ws")
        .finish();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(graphql).service(graphql_ws).service(graphiql);
}
