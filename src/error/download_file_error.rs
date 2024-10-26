use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DownloadFileError {
    MediaMessageNotFound,
    MetadataMessageIsNotFound,
    CannotDeserializeFileMetadataFromMessage,
    CannotDownloadMediaFromMessage, 
    FileHasNotGotNameInMetadata
}

impl std::error::Error for DownloadFileError {}

impl Display for DownloadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadFileError::MediaMessageNotFound => write!(f, "Media message not found."),
            DownloadFileError::MetadataMessageIsNotFound => write!(f, "Metadata message is not found."),
            DownloadFileError::CannotDeserializeFileMetadataFromMessage => write!(f, "Cannot deserialize metadata from message."),
            DownloadFileError::CannotDownloadMediaFromMessage => write!(f, "Cannot download media from message."),
            DownloadFileError::FileHasNotGotNameInMetadata => write!(f, "File has not got name in metadata."),
        }
    }
}

