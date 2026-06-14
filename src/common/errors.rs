#[derive(Debug)]
pub enum LlaasError {
    TtsError(String),
    ModelInitializationError(String),
    PredictionError(String),
    FileNotFound(String),
    IOError(std::io::Error),
    DatabaseError(surrealdb::Error),
}

/**
 * Implements the From trait to convert a standard I/O error into a LlaasError.
 */
impl From<std::io::Error> for LlaasError {
    fn from(error: std::io::Error) -> Self {
        LlaasError::IOError(error)
    }
}

/**
 * Implements the From trait to convert a SurrealDB error into a LlaasError.
 */
impl From<surrealdb::Error> for LlaasError {
    fn from(error: surrealdb::Error) -> Self {
        LlaasError::DatabaseError(error)
    }
}