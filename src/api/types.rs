use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate,Debug, Serialize, Deserialize)]
pub struct Language {

    #[validate(length(min = 2, max = 10))]
    pub name: String,
    #[validate(length(min = 2, max = 10))]
    pub code: String,
}