use serde::{Deserialize, Serialize};

use crate::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
}

impl FileMetadata {
    pub fn new(path: Path) -> Self {
        Self {
            path: path.to_string()
        }
    }
}
