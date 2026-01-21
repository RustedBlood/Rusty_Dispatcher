use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreErrors {
    #[error("No permission to read files")]
    NoPermission,
    #[error("IO error {0}")]
    Io(#[from] std::io::Error),
}
