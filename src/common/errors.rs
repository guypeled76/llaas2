use any_tts::TtsError;



/// This module defines the common error types used across the Llaas application.
#[derive(Debug)]
pub enum Error {
    ErrorMessage(String),
    IOError(IOInfo),
    DBError(DatabaseInfo),
    TtsError(TtsInfo),
}

#[derive(Debug)]
pub struct IOInfo(pub String, pub Option<std::io::Error>);


#[derive(Debug)]
pub struct DatabaseInfo(pub String, pub Option<surrealdb::Error>);

#[derive(Debug)]
pub struct TtsInfo(pub String, pub Option<TtsError>);

/**
 * Implements the From trait to convert a standard I/O error into a LlaasError.
 */
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(IOInfo(format!("I/O error occurred: {}", error), Some(error)))  
    }
}


/**
 * Implements the From trait to convert a SurrealDB error into a LlaasError.
 */
impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        Error::DBError(DatabaseInfo(format!("Database error occurred: {}", error), Some(error)))
    }
}

/**
 * Implements the From trait to convert a TTS error into a LlaasError.
 */
impl From<TtsError> for Error {
    fn from(error: TtsError) -> Self {
        Error::TtsError(TtsInfo(format!("TTS error occurred: {}", error), Some(error)))
    }
}
