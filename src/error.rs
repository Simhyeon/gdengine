use thiserror::Error;
use rad::error::RadError;

#[derive(Debug, Error)]
pub enum GdeError {
    #[error("IO Error : {0}")]
    IoError(std::io::Error),
    #[error("Failed to add source script for macro execution")]
    SourceError,
    #[error("Failed to operate json : {0}")]
    JsonError(serde_json::Error),
    #[error("Config error : {0}")]
    ConfigError(String),
    #[error("Not a gde directory")]
    NotGdeDirectory,
    #[error("Raderror : {0}")]
    Raderror(RadError),
}

impl From<std::io::Error> for GdeError {
    fn from(err : std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for GdeError {
    fn from(err : serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl From<RadError> for GdeError {
    fn from(err : RadError) -> Self {
        Self::Raderror(err)
    }
}
