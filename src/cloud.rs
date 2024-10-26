use grammers_client::grammers_tl_types as tl;

use crate::error::*;
use crate::path::Path;
use crate::types::file::File;
use crate::types::file::FileMetadata;

use crate::types::file::FileIter;

#[derive(Clone, Debug)]
pub struct Cloud {
    pub client: grammers_client::Client,
}

impl Cloud {
    pub async fn get_storage_chat(
        &self,
        storage_chat_name: &str,
    ) -> Option<grammers_client::types::Chat> {
        let mut dialogs_iter = self.client.iter_dialogs();

        while let Ok(Some(dialog)) = dialogs_iter.next().await {
            match dialog.chat() {
                grammers_client::types::Chat::Group(group) => {
                    if group.title() == storage_chat_name {
                        return Some(dialog.chat.into());
                    }
                }
                _ => continue,
            }
        }
        None
    }

    pub async fn create_storage_chat(
        &self,
        storage_chat_name: &str,
    ) -> Result<grammers_client::types::Chat, CreateStorageChatError> {
        match self
            .client
            .invoke(&tl::functions::messages::CreateChat {
                users: vec![tl::enums::InputUser::UserSelf],
                title: storage_chat_name.to_string(),
                ttl_period: None,
            })
            .await
        {
            Ok(_) => self
                .get_storage_chat(storage_chat_name)
                .await
                .ok_or(CreateStorageChatError::CreatedStorageChatIsNotFound),
            Err(_) => Err(CreateStorageChatError::ChatIsNotCreated),
        }
    }

    pub async fn delete_storage_chat(
        &self,
        storage_chat: &grammers_client::types::Chat,
    ) -> Result<bool, DeleteStorageChatError> {
        self.client
            .invoke(&tl::functions::messages::DeleteChat {
                chat_id: storage_chat.id(),
            })
            .await
            .map_err(|_| DeleteStorageChatError::CannotDeleteChat)
    }

    pub fn file_iter(&self, storage_chat: &grammers_client::types::Chat) -> FileIter {
        FileIter::new(self.client.clone(), storage_chat.clone())
    }

    pub async fn upload_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        current_path: P,
        saving_path: &Path,
    ) -> Result<File, UploadFileError> {
        let uploaded_file = self
            .client
            .upload_file(&current_path)
            .await
            .map_err(|_| UploadFileError::CannotUploadFile)?;

        let media_message = self
            .client
            .send_message(
                chat,
                grammers_client::InputMessage::text("").file(uploaded_file),
            )
            .await
            .map_err(|_| UploadFileError::MediaMessageIsNotSended)?;

        let metadata = FileMetadata {
            path: saving_path
                .join(
                    Into::<Path>::into(current_path.as_ref())
                        .name()
                        .ok_or(UploadFileError::FileHasNotGotName)?,
                )
                .to_string(),
            file_message_id: media_message.id(),
        };

        let metadata_message = self
            .client
            .send_message(
                chat,
                grammers_client::InputMessage::text(
                    serde_json::to_string(&metadata)
                        .map_err(|_| UploadFileError::CannotSendMetadata)?,
                ),
            )
            .await
            .map_err(|_| UploadFileError::MediaMessageIsNotSended)?;
        Ok(File::new(metadata, metadata_message.id()))
    }

    pub async fn download_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        file: File,
        save_path: P,
    ) -> Result<(), DownloadFileError> {
        let message = self
            .client
            .get_messages_by_id(chat, &[file.message_id])
            .await
            .map_err(|_| DownloadFileError::MetadataMessageIsNotFound)?;

        if let Some(Some(message)) = message.first() {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                if let Some(Some(media_message)) = self
                    .client
                    .get_messages_by_id(chat, &[file_metadata.file_message_id])
                    .await
                    .map_err(|_| DownloadFileError::MediaMessageNotFound)?
                    .first()
                {
                    let path = Path::new(&file_metadata.path);

                    media_message
                        .download_media(format!(
                            "{}/{}",
                            save_path.as_ref().display(),
                            path.name()
                                .ok_or(DownloadFileError::FileHasNotGotNameInMetadata)?
                        ))
                        .await
                        .map_err(|_| DownloadFileError::CannotDownloadMediaFromMessage)?;
                } else {
                    return Err(DownloadFileError::MediaMessageNotFound);
                }
            } else {
                return Err(DownloadFileError::CannotDeserializeFileMetadataFromMessage);
            }
        }

        Ok(())
    }

    pub async fn delete_file(
        &self,
        chat: &grammers_client::types::Chat,
        file: File,
    ) -> Result<(), DeleteFileError> {
        if let Some(Some(message)) = self
            .client
            .get_messages_by_id(chat, &[file.message_id])
            .await
            .map_err(|_| DeleteFileError::MessageIsNotFound)?
            .first()
        {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                self.client
                    .delete_messages(chat, &[file_metadata.file_message_id, file.message_id])
                    .await
                    .map_err(|_| DeleteFileError::CannotDeleteFile)?;
            }
            Ok(())
        } else {
            Err(DeleteFileError::MessageIsNotFound)
        }
    }
}
