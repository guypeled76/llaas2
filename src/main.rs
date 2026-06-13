mod models;

use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionModel, 
    zero_shot_classification::ZeroShotClassificationModel
};





fn main() {
    models::translate::apply();
    models::pos::apply();
    models::sentiment::apply();
    models::keywords::apply();
    models::classification::apply();
}











