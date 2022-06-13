use r4d::RadError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GdeError {
    #[error("No such file : {0}")]
    NoSuchPath(String),
    #[error("IO Error : {0}")]
    IoError(std::io::Error),
    #[error("Renderer error : {0}")]
    RendererError(&'static str),
    #[error("Failed to operate json : {0}")]
    JsonError(serde_json::Error),
    #[error("Config error : {0}")]
    ConfigError(String),
    #[error("Not a gde directory")]
    NotGdeDirectory,
    #[error("Raderror : {0}")]
    Raderror(RadError),
    #[error("Reqwest error : {0}")]
    ReqError(reqwest::Error),
    #[error("Invalid command error : {0}")]
    InvalidCommand(String),
    #[error("Invalid variable file : {0}")]
    VarFileError(String),
    #[error("Failed plotting operation : {0}")]
    PlotError(String),
}

impl From<std::io::Error> for GdeError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for GdeError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}

impl From<RadError> for GdeError {
    fn from(err: RadError) -> Self {
        Self::Raderror(err)
    }
}

impl From<reqwest::Error> for GdeError {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqError(err)
    }
}
