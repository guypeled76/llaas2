use rust_bert::pipelines::{
    zero_shot_classification::ZeroShotClassificationModel
};


pub fn apply() {
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