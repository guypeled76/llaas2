use any_tts::{ModelType, SynthesisRequest, TtsConfig, TtsModel as AnyTtsModel, load_model};

use llaas_common::errors::Error;

/**
 * An enumeration to define preset configurations for TTS models.
 * This allows for easy selection of different TTS backends without needing to specify detailed configuration parameters each time.
 * The presets can be expanded in the future to include additional models or configurations as needed.
 */
pub enum TtsPreset {
    Kokoro,
    OmniVoice,
}

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
    pub fn new(settings: TtsPreset) -> Result<TtsModel, Error> {
        let config = match settings {
            TtsPreset::Kokoro => TtsConfig::new(ModelType::Kokoro).with_preferred_runtime(),
            TtsPreset::OmniVoice => TtsConfig::new(ModelType::OmniVoice).with_preferred_runtime(),
        };
        let model = load_model(config)?;
        Ok(TtsModel { model: model })
    }

    /**
     * Synthesizes speech for the given text and returns WAV bytes.
     */
    pub fn synthesize(&self, text: &str, lang: &str) -> Result<Vec<u8>, Error> {
        let request = SynthesisRequest::new(text).with_language(lang);
        let audio = self.model.synthesize(&request)?;
        Ok(audio.get_wav())
    }
}

/**
 * A convenience method to synthesize speech in one call.
 * It initializes the model and returns generated WAV bytes.
 */
pub fn as_wav(settings: TtsPreset, text: &str, lang: &str) -> Result<Vec<u8>, Error> {
    let model = TtsModel::new(settings)?;
    model.synthesize(text, lang)
}
/**
 * A convenience method to synthesize speech and write the WAV bytes to a file.
 */
pub fn save_as_wav(settings: TtsPreset, text: &str, file: &str, lang: &str) -> Result<(), Error> {
    let wav_bytes = as_wav(settings, text, lang)?;
    std::fs::write(file, wav_bytes)?;
    Ok(())
}
