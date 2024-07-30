use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DownloadFileError {
    CannotGetMessageInStorageChat,
    CannotDeserializeFileMetadataFromMessage,
    CannotDownloadMediaFromMessage, 
    FileHasNotGotNameInMetadata
}

impl std::error::Error for DownloadFileError {}

impl Display for DownloadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadFileError::CannotGetMessageInStorageChat => write!(f, "Cannot get message in storage chat."),
            DownloadFileError::CannotDeserializeFileMetadataFromMessage => write!(f, "Cannot deserialize metadata from message."),
            DownloadFileError::CannotDownloadMediaFromMessage => write!(f, "Cannot download media from message."),
            DownloadFileError::FileHasNotGotNameInMetadata => write!(f, "File has not got name in metadata."),
        }
    }
}

