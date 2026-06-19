use actix_web::web;
use async_graphql::{Context, Object};

use crate::common::context::Context as AppContext;

/// Root query object for GraphQL queries.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns the name of the service.
    async fn service_name(&self) -> &str {
        "llaas"
    }

    /// Returns a list of available language codes.
    ///
    /// Access to app context is available through GraphQL context data.
    async fn languages(&self, ctx: &Context<'_>) -> Vec<&str> {
        let context = ctx.data::<web::Data<&'static AppContext>>().ok();
        let _ = context;
        vec!["en", "es", "fr", "de"]
    }
}
