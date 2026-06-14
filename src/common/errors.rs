#[derive(Debug)]
pub enum LlaasError {
    TtsError(String),
    ModelInitializationError(String),
    PredictionError(String),
    FileNotFound(String),
    IOError(std::io::Error),
}

/**
 * Implements the From trait to convert a standard I/O error into a LlaasError.
 */
impl From<std::io::Error> for LlaasError {
    fn from(error: std::io::Error) -> Self {
        LlaasError::IOError(error)
    }
}