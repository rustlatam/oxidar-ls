#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("there was an error reading a file: {0}")]
    UnableToReadFile(anyhow::Error),

    #[error("there was an error listing files: {0}")]
    ErrorListingFiles(anyhow::Error),
}
