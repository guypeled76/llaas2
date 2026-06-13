use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionModel
};

pub fn apply() {
    let model = KeywordExtractionModel::new(Default::default()).unwrap();
    let input = [
        "Rust is a systems programming language focused on safety and performance.", 
        "The quick brown fox jumps over the lazy dog."
    ];
    let output = model.predict(&input);
    
    for(i, keywords) in output.iter().enumerate() {
        println!("Sentence {}: Keywords: {:?}", i + 1, keywords);
    }
}