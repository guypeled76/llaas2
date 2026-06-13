pub fn apply() {
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