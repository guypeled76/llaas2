use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionModel
};

pub struct KeywordsModel {
    model: KeywordExtractionModel<'static>,
}



impl KeywordsModel {

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

    pub fn predict(&self, text: &[&str]) -> Vec<String> {
        self.model.predict(text)
                .into_iter()
                .flatten()
                .flatten()
                .map(|kw| kw.text)
                .collect()
    }

    pub fn apply(text: &[&str]) -> Vec<String> {
        let model = KeywordsModel::new().expect("Failed to initialize the model");
        model.predict(text)
    }
}


