use crate::path::Path;

use super::FileMetadata;

#[derive(Debug, Clone)]
pub struct File {
    pub path: Path,
    pub message_id: i32,
}

impl File {
    pub(crate) fn new(metadata: FileMetadata, message_id: i32) -> Self {
        Self {
            path: metadata.path.into(),
            message_id,
        }
    }
}
