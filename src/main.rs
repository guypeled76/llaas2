mod models;

use rust_bert::pipelines::{
    keywords_extraction::KeywordExtractionModel, 
    zero_shot_classification::ZeroShotClassificationModel
};





fn main() {
    models::translate::apply();
    pos();
    sentiment();
    keywords();
    clasification();
}




fn pos() {
    let model = rust_bert::pipelines::pos_tagging::POSModel::new(Default::default()).unwrap();
    let input = [
        "Are you sure this is working?", 
        "本当にこれでうまくいってますか？",
        "Er du sikker på, at dette virker?",
        "¿Estás seguro de que esto funciona?"
    ];
    let output = model.predict(&input);
    
    for(i, sentence) in output.iter().enumerate() {
        println!("Sentence {}:", i + 1);
        for (word, tag) in sentence.iter().enumerate() {
            println!("{}:{} - {} ({})", word + 1, tag.word, tag.label, tag.score);
        }
    }
}

fn sentiment() {
    let model = rust_bert::pipelines::sentiment::SentimentModel::new(Default::default()).unwrap();
    let input = [
        "I love this product!", 
        "This is the worst experience I've ever had."
    ];
    let output = model.predict(&input);
    
    for(i, sentiment) in output.iter().enumerate() {
        println!("Sentence {}: {:#?} (score: {})", i + 1, sentiment.polarity, sentiment.score);
    }
}

fn keywords() {
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

fn clasification() {
    let classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    let input = [
        "I love this movie!", 
        "This is the worst experience I've ever had."
    ];
    let candidate_labels = ["positive", "negative", "neutral"];
    let output = classification_model.predict(
        &input, 
        &candidate_labels, 
        None, 
        16
    );   
    for(i, classification) in output.iter().enumerate() {
        println!("Sentence {}: {:#?}", i + 1, classification);
    }

    
}

