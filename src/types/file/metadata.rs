use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FileMetadata {
    pub path: String,
    pub file_message_id: i32,
}
