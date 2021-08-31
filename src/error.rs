use thiserror::Error;

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
