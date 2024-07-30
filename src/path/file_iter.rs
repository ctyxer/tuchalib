use super::{File, FileMetadata};

pub struct FileIter {
    pub(crate) message_iter: grammers_client::client::messages::MessageIter
}

impl FileIter {
    pub(crate) fn new(message_iter: grammers_client::client::messages::MessageIter) -> Self {
        Self {
            message_iter
        }
    }

    pub async fn next(&mut self) -> Option<File> {
        while let Ok(Some(message)) = self.message_iter.next().await {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                return Some(File::new(file_metadata, message.id()));
            }
        }
        None
    }

    pub async fn collect(&mut self) -> Vec<File> {
        let mut vector = Vec::<File>::new();

        while let Some(file) = self.next().await {
            vector.push(file);
        }

        vector
    }
}