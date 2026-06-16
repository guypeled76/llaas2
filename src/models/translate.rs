use rust_bert::pipelines::{
    common::ModelType,
    translation::{Language, TranslationModelBuilder},
};

use tch::Device;

pub fn apply() {
    let model = TranslationModelBuilder::new()
        .with_device(Device::cuda_if_available())
        .with_model_type(ModelType::Marian)
        .with_source_languages(vec![Language::English])
        .with_target_languages(vec![Language::Spanish])
        .create_model()
        .unwrap();

    let input = [
        "Hello, how are you?",
        "This is a test of the translation pipeline.",
    ];

    let output = model.translate(&input, None, Language::Spanish);

    match output {
        Ok(translations) => {
            for v in translations {
                println!("{}", v);
            }
        }
        Err(e) => {
            eprintln!("Error during translation: {}", e);
            return;
        }
    }
}
