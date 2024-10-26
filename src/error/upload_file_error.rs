#[derive(Debug, Clone)]
pub enum UploadFileError {
    FileHasNotGotName,
    CannotSendMetadata,
    CannotUploadFile,
    MediaMessageIsNotSended,
    MetadataMessageIsNotSended,
}

impl std::error::Error for UploadFileError {}

impl std::fmt::Display for UploadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UploadFileError::CannotUploadFile => write!(f, "Cannot upload file."),
            UploadFileError::FileHasNotGotName => write!(f, "File has not got name."),
            UploadFileError::CannotSendMetadata => write!(f, "Cannot send metadata."),
            UploadFileError::MediaMessageIsNotSended => write!(f, "Media message is not sended."),
            UploadFileError::MetadataMessageIsNotSended => write!(f, "Metadata message is not sended."),
        }
    }
}
