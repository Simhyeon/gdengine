use thiserror::Error;

#[derive(Debug, Error)]
pub enum GdeError {
    #[error("IO Error : {0}")]
    IoError(std::io::Error),
    #[error("Failed to add source script for macro execution")]
    SourceError
}

impl From<std::io::Error> for GdeError {
    fn from(err : std::io::Error) -> Self {
        Self::IoError(err)
    }
}
