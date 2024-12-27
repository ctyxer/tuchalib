use crate::error::*;
use crate::path::Path;
use crate::types::file::File;
use crate::types::file::FileMetadata;

use crate::types::file::FileIter;

#[trait_variant::make]
pub trait ClientExt {
    fn iter_files<C: Into<grammers_client::types::PackedChat>>(&self, chat: C) -> FileIter;

    async fn upload_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        current_path: P,
        saving_path: &Path,
    ) -> Result<File, UploadFileError>;

    async fn download_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        file: File,
        save_path: P,
    ) -> Result<(), DownloadFileError>;

    async fn delete_file(
        &self,
        chat: &grammers_client::types::Chat,
        file: File,
    ) -> Result<(), DeleteFileError>;
}

impl ClientExt for grammers_client::Client {
    fn iter_files<C: Into<grammers_client::types::PackedChat>>(&self, chat: C) -> FileIter {
        FileIter::new(self.clone(), chat.into())
    }

    async fn upload_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        current_path: P,
        saving_path: &Path,
    ) -> Result<File, UploadFileError> {
        let uploaded_file = self
            .upload_file(&current_path)
            .await
            .map_err(|_| UploadFileError::CannotUploadFile)?;

        let media_message = self
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

    async fn download_file<P: AsRef<std::path::Path>>(
        &self,
        chat: &grammers_client::types::Chat,
        file: File,
        save_path: P,
    ) -> Result<(), DownloadFileError> {
        let message = self
            .get_messages_by_id(chat, &[file.message_id])
            .await
            .map_err(|_| DownloadFileError::MetadataMessageIsNotFound)?;

        if let Some(Some(message)) = message.first() {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                if let Some(Some(media_message)) = self
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

    async fn delete_file(
        &self,
        chat: &grammers_client::types::Chat,
        file: File,
    ) -> Result<(), DeleteFileError> {
        if let Some(Some(message)) = self
            .get_messages_by_id(chat, &[file.message_id])
            .await
            .map_err(|_| DeleteFileError::MessageIsNotFound)?
            .first()
        {
            if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(message.text()) {
                self.delete_messages(chat, &[file_metadata.file_message_id, file.message_id])
                    .await
                    .map_err(|_| DeleteFileError::CannotDeleteFile)?;
            }
            Ok(())
        } else {
            Err(DeleteFileError::MessageIsNotFound)
        }
    }
}
