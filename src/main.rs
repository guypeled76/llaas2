use rust_bert::pipelines::{common::ModelType, translation::{
    Language,  
    TranslationModelBuilder,
}};

use tch::{Device};

fn main() {
    translate();
    pos();
}


fn translate() {
    let model = TranslationModelBuilder::new()
        .with_device(Device::cuda_if_available())
        .with_model_type(ModelType::Marian)
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish])
        .create_model()
        .unwrap();

    let input = [
        "Hello, how are you?", 
        "This is a test of the translation pipeline."
    ];
    
    let output = model.translate(
        &input, 
        None, 
        Language::Spanish
    );

    match output {
        Ok(translations) => for v in translations {
            println!("{}", v);
        },
        Err(e) => {
            eprintln!("Error during translation: {}", e);
            return;
        }
    }
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