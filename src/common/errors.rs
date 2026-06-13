#[derive(Debug)]
pub enum LlaasError {
    EpubReadError(String),
    TtsError(String),
    ModelInitializationError(String),
    PredictionError(String),
}