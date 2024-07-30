use grammers_tl_types as tl;
use grammers_tl_types::functions;

use crate::error::*;
use crate::path::FileIter;
use crate::path::FileMetadata;
use crate::path::Path;

#[derive(Clone, Debug)]
pub struct Cloud<'a> {
    client: &'a grammers_client::Client,
}

impl<'a> Cloud<'a> {
    pub fn new(client: &'a grammers_client::Client) -> Self {
        Self { client }
    }

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
            .invoke(&functions::messages::CreateChat {
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

    pub async fn uploaded_file(&self, chat: &grammers_client::types::Chat) -> FileIter {
        FileIter::new(self.client.iter_messages(chat))
    }

    pub async fn upload_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        file: P,
        saving_path: Path,
    ) -> Result<(), UploadFileError> {
        dbg!(Into::<Path>::into(file.as_ref()).components());
        let metadata = FileMetadata::new(
            saving_path.join(
                Into::<Path>::into(file.as_ref())
                    .name()
                    .ok_or(UploadFileError::FileHasNotGotName)?,
            ),
        );

        let input_message = grammers_client::InputMessage::text(
            serde_json::to_string(&metadata).map_err(|_| UploadFileError::CannotSendMetadata)?,
        )
        .document(
            self.client
                .upload_file(file.as_ref())
                .await
                .map_err(|_| UploadFileError::CannotUploadFile)?,
        );

        if self.client.send_message(chat, input_message).await.is_err() {
            return Err(UploadFileError::MediaMessageIsNotSended);
        }
        Ok(())
    }

    pub async fn download_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        message_id: i32,
        save_path: P,
    ) -> Result<(), DownloadFileError> {
        let message = self
            .client
            .get_messages_by_id(chat, &[message_id])
            .await
            .map_err(|_| DownloadFileError::CannotGetMessageInStorageChat)?;

        for message in message {
            if let Some(message) = message {
                if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                    let path = Path::new(&file_metadata.path);

                    if message
                        .download_media(format!(
                            "{}/{}",
                            save_path.as_ref().display(),
                            path.name()
                                .ok_or(DownloadFileError::FileHasNotGotNameInMetadata)?
                        ))
                        .await
                        .is_err()
                    {
                        return Err(DownloadFileError::CannotDownloadMediaFromMessage);
                    }
                } else {
                    return Err(DownloadFileError::CannotDeserializeFileMetadataFromMessage);
                }
            }
        }

        Ok(())
    }

    pub async fn delete_file(
        &self,
        chat: &grammers_client::types::Chat,
        message_id: i32,
    ) -> Result<usize, DeleteFileError> {
        self.client
            .delete_messages(chat, &[message_id])
            .await
            .map_err(|_| DeleteFileError::CannotDeleteFile)
    }
}
