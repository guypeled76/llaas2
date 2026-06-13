use any_tts::{
    load_model,
    ModelType,
    SynthesisRequest,
    TtsConfig,
    TtsModel as AnyTtsModel,
};

use crate::common::errors::LlaasError;
    
/**
 * A wrapper around any-tts to handle model initialization and speech synthesis.
 * This struct provides a simple interface to synthesize WAV audio from text.
 */
pub struct TtsModel {
    model: Box<dyn AnyTtsModel>,
}

impl TtsModel {
    /**
     * Initializes the TTS model with a Kokoro backend.
     * If model loading fails, it returns an error message.
     */
    pub fn new() -> Result<TtsModel, LlaasError> {
        let config = TtsConfig::new(ModelType::OmniVoice);
        let result = load_model(config);

        match result {
            Ok(model) => Ok(TtsModel { model }),
            Err(e) => {
                eprintln!("Error initializing TTS model: {}", e);
                Err(LlaasError::ModelInitializationError(e.to_string()))
            }
        }
    }

    /**
     * Synthesizes speech for the given text and returns WAV bytes.
     */
    pub fn predict(&self, text: &str, lang: &str) -> Result<Vec<u8>, LlaasError> {
        let request = SynthesisRequest::new(text).with_language(lang);
        let audio = self.model.synthesize(&request).map_err(|e| LlaasError::PredictionError(e.to_string()))?;
        Ok(audio.get_wav())
    }

    /**
     * A convenience method to synthesize speech in one call.
     * It initializes the model and returns generated WAV bytes.
     */
    pub fn apply(text: &str, lang: &str) -> Result<Vec<u8>, LlaasError> {
        let model = TtsModel::new()?;
        model.predict(text, lang)
    }

    /**
     * A convenience method to synthesize speech and write the WAV bytes to a file.
     */
    pub fn wav(text: &str, file: &str, lang: &str) -> Result<(), LlaasError> {
        let wav_bytes = TtsModel::apply(text, lang)?;
        std::fs::write(file, wav_bytes).map_err(|e| LlaasError::TtsError(e.to_string()))
    }
}
