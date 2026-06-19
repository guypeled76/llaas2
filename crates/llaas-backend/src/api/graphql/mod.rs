//! GraphQL API implementation for LLAAS.
//!
//! This module provides the GraphQL schema and HTTP handlers
//! for the Language Learning as a Service (LLAAS) backend.

mod root;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

pub use root::QueryRoot;

/// The complete GraphQL schema for LLAAS.
///
/// Defines the structure with:
/// - `QueryRoot`: All available queries
/// - `EmptyMutation`: No mutations currently implemented
/// - `EmptySubscription`: No subscriptions currently implemented
pub type LlaasSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Creates and returns a new instance of the LLAAS GraphQL schema.
///
/// # Returns
/// A fully configured [`LlaasSchema`] ready to execute GraphQL queries.
pub fn schema() -> LlaasSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}

/// Handles GraphQL POST requests.
///
/// This endpoint accepts GraphQL queries and mutations in JSON format
/// and returns the results.
#[post("/graphql")]
async fn graphql(schema: web::Data<LlaasSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

/// Handles GraphQL WebSocket subscriptions.
#[get("/graphql/ws")]
async fn graphql_ws(
    schema: web::Data<LlaasSchema>,
    request: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(schema.get_ref().clone()).start(&request, payload)
}

/// Serves the GraphiQL interactive IDE.
#[get("/graphql/ui")]
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

/// Registers all GraphQL routes with the Actix web service configuration.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(graphql).service(graphql_ws).service(graphiql);
}
