#[derive(Debug, Clone)]
pub enum UploadedFileError {
    InvocationError,
    MessageIsNotAFile
}

impl std::error::Error for UploadedFileError {}

impl std::fmt::Display for UploadedFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UploadedFileError::InvocationError => write!(f, "Invocation error."),
            UploadedFileError::MessageIsNotAFile => write!(f, "Message is not a file."),
        }
    }
}