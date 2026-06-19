use async_graphql::{Context, Object};

use crate::common::context::Context as AppContext;
use crate::store::videos::{Video, VideoDatabase};

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
        let context = ctx.data::<&'static AppContext>().ok();
        let _ = context;
        vec!["en", "es", "fr", "de"]
    }

    /// Returns all videos stored in the database.
    async fn videos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Video>> {
        let context = *ctx.data::<&'static AppContext>()?;
        let database: &'static dyn VideoDatabase = context;

        database
            .videos()
            .await
            .map_err(|err| async_graphql::Error::new(format!("Failed to load videos: {:?}", err)))
    }
}


