use crate::context::Context;

use crate::common::errors::Error;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, SurrealValue};

/**
 * The Video struct represents a video resource that has been downloaded from a given URL.
 * It contains fields for the URL of the video, the path to the downloaded video file,
 * the paths to the extracted subtitles, and the list of languages for which subtitles were extracted.
 * The video field is a tuple containing the path to the downloaded video file and a boolean indicating whether the file exists and is valid.
 * The subtitles field is a vector of tuples, each containing the language, path to the subtitle file, and a boolean indicating whether the file exists and is valid.
 * The languages field is a vector of strings representing the languages for which subtitles were extracted
 */
#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub url: String,
    pub video: String,
    pub subtitles: Vec<String>,
    pub languages: Vec<String>,
}

#[async_trait::async_trait]
pub trait VideoDatabase {
    async fn upsert(&self, video: &Video) -> Result<(), Error>;
}

#[async_trait::async_trait]
impl VideoDatabase for Context {
    async fn upsert(&self, video: &Video) -> Result<(), Error> {
        // Here you would implement the logic to insert or update the video record in your database.
        // This is a placeholder implementation and should be replaced with actual database interaction code.

        let id = match &video.id {
            Some(id) => id,
            None => Err(Error::ErrorMessage(
                "Video ID is required for upsert operation.".into(),
            ))?,
        };

        // Get the database connection from the context.
        let db = self.db().await?;

        println!("Upserting video with URL: {}", video.url);
        let result: Option<Video> = db.upsert(id).content(video.clone()).await?;
        println!("Video upserted: {:?}", result);

        Ok(())
    }
}
