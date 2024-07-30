#[derive(Debug, Clone)]
pub enum DeleteFileError {
    CannotDeleteFile,
}

impl std::error::Error for DeleteFileError {}

impl std::fmt::Display for DeleteFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteFileError::CannotDeleteFile => write!(f, "Cannot delete file."),
        }
    }
}
