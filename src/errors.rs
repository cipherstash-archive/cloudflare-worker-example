use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum RequestError {
    #[error("ID missing or invalid")]
    InvalidID,
    #[error("No such collection")]
    NoSuchCollection,
    #[error("No such record")]
    NoSuchRecord,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Work is missing well formed collection ID in config")]
    MissingCollectionId,
    #[error("Config error: {0}")]
    ConfigError(String),
}

// TODO: Rename Request Error
impl From<RequestError> for worker::Error {
    fn from(error: RequestError) -> Self {
        worker::Error::RustError(error.to_string())
    }
}