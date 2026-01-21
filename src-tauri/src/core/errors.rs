use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreErrors {
    #[error("No permission to read files")]
    NoPermission,
    #[error("IO error {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for CoreErrors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // Serialize the error as a string for the frontend
        serializer.serialize_str(self.to_string().as_ref())
    }
}
