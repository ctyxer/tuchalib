#[derive(Debug, Clone)]
pub enum DeleteFileError {
    CannotDeleteFile,
    MessageIsNotFound,
}

impl std::error::Error for DeleteFileError {}

impl std::fmt::Display for DeleteFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteFileError::MessageIsNotFound => write!(f, "Message is not found."),
            DeleteFileError::CannotDeleteFile => write!(f, "Cannot delete file."),
        }
    }
}
