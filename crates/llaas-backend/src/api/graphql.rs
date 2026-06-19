//! GraphQL API implementation for LLAAS.
//!
//! This module provides the GraphQL schema, resolvers, and HTTP handlers
//! for the Language Learning as a Service (LLAAS) backend.

use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use async_graphql::http::GraphiQLSource;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

use crate::common::context::Context as AppContext;

/// Root query object for the GraphQL schema.
///
/// Defines all available top-level queries that clients can execute.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the name of the service.
    ///
    /// # Returns
    /// A static string identifying the service as "llaas".
    async fn service_name(&self) -> &str {
        "llaas"
    }

    /// Returns a list of available language codes.
    ///
    /// This query provides information about which languages are currently
    /// supported by the LLAAS service.
    ///
    /// # Arguments
    /// - `ctx`: The GraphQL context, which contains the application context with database and configuration
    ///
    /// # Returns
    /// A vector of language codes as strings (e.g., "en", "es", "fr", "de").
    async fn languages(&self, ctx: &Context<'_>) -> Vec<&str> {
        let context = ctx.data::<web::Data<&'static AppContext>>().ok();
        let _ = context; // Use the context as needed
        vec!["en", "es", "fr", "de"]
    }
}

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
///
/// # Arguments
/// - `schema`: The GraphQL schema instance used to execute requests
/// - `request`: The incoming GraphQL request from the client
///
/// # Returns
/// A GraphQL response containing either the query result or error details
#[post("/graphql")]
async fn graphql(schema: web::Data<LlaasSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

/// Handles GraphQL WebSocket subscriptions.
///
/// This endpoint upgrades an HTTP connection to a WebSocket, allowing
/// clients to receive real-time updates via GraphQL subscriptions.
///
/// # Arguments
/// - `schema`: The GraphQL schema instance used to execute subscriptions
/// - `request`: The HTTP upgrade request
/// - `payload`: The WebSocket payload stream
///
/// # Returns
/// An HTTP response handling the WebSocket upgrade, or an error if the upgrade fails
#[get("/graphql/ws")]
async fn graphql_ws(
    schema: web::Data<LlaasSchema>,
    request: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(schema.get_ref().clone()).start(&request, payload)
}

/// Serves the GraphiQL interactive IDE.
///
/// This endpoint provides a web-based interface for exploring and testing
/// the GraphQL schema. Useful for development and debugging.
///
/// # Returns
/// An HTML response containing the GraphiQL IDE configured to communicate
/// with the GraphQL endpoint at `/graphql` and subscriptions at `/graphql/ws`
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
///
/// This function should be called during server initialization to mount
/// the GraphQL endpoints on the application.
///
/// # Routes Registered
/// - `POST /graphql`: GraphQL query/mutation endpoint
/// - `GET /graphql/ws`: GraphQL WebSocket subscription endpoint
/// - `GET /admin/graphql`: GraphiQL interactive IDE
///
/// # Arguments
/// - `cfg`: The Actix service configuration builder to register routes with
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(graphql).service(graphql_ws).service(graphiql);
}