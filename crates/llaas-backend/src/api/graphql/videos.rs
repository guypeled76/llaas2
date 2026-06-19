use async_graphql::Object;
use crate::store::videos::Video;

#[Object]
impl Video {
    async fn id(&self) -> Option<String> {
        match  self.id.as_ref().map(|id| id.key.clone()) {
            Some(key) => match key {
                surrealdb_types::RecordIdKey::String(id) => Some(id),
                surrealdb_types::RecordIdKey::Number(id) => Some(id.to_string()),
                surrealdb_types::RecordIdKey::Uuid(id) => Some(id.to_string()),
                surrealdb_types::RecordIdKey::Object(_) => None,
                surrealdb_types::RecordIdKey::Array(_) => None,
                surrealdb_types::RecordIdKey::Range(_) => None,
            },
            None => None,
        }
    }

    async fn url(&self) -> &str {
        &self.url
    }

    async fn video(&self) -> &str {
        &self.video
    }

    async fn subtitles(&self) -> &[String] {
        &self.subtitles
    }

    async fn languages(&self) -> &[String] {
        &self.languages
    }
}