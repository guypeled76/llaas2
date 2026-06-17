use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct LanguageRequest {
    #[validate(length(min = 2, max = 10))]
    pub name: String,
    #[validate(length(min = 2, max = 10))]
    pub code: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct LanguageUrl {
    #[validate(length(min = 2, max = 10))]
    pub code: String,
}
