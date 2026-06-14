#[derive(Debug)]
pub enum LlaasError {
    TtsError(String),
    ModelInitializationError(String),
    PredictionError(String),
}

