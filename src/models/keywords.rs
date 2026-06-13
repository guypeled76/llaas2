use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionModel
};

/**
 * A wrapper around the KeywordExtractionModel to handle initialization and prediction.
 * This struct provides a simple interface to extract keywords from given text inputs.
 */
pub struct KeywordsModel {
    model: KeywordExtractionModel<'static>,
}



impl KeywordsModel {

    /**
     * Initializes the KeywordExtractionModel. If the model fails to load, it returns an error message.
     */
    pub fn new() -> Result<KeywordsModel, String> {
        let result = KeywordExtractionModel::new(Default::default());
        
        match result {
            Ok(model) =>  Ok(KeywordsModel { model }) ,
            Err(e) => {
                eprintln!("Error initializing the model: {}", e);
                Err(e.to_string()) 
            }
        }
    }

    /**
     * Predicts keywords from the given text inputs. It processes the input text and returns a vector of extracted keywords.
     */
    pub fn predict(&self, text: &[&str]) -> Vec<String> {
        self.model.predict(text)
                .into_iter()
                .flatten()
                .flatten()
                .map(|kw| kw.text)
                .collect()
    }

    /**
     * A convenience method to apply the keyword extraction directly on an array of text inputs. It initializes the model and returns the extracted keywords.
     */
    pub fn apply(text: &[&str]) -> Vec<String> {
        let model = KeywordsModel::new().expect("Failed to initialize the model");
        model.predict(text)
    }
}


