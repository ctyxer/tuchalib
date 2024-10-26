use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;
use grammers_client::{
    client::messages::MessageIter,
    types::{Chat, Message},
    Client,
};
use grammers_client::InvocationError;

use super::{File, FileMetadata};

pub struct FileIter {
    next_message:
        Pin<Box<dyn Future<Output = (Result<Option<Message>, InvocationError>, MessageIter)>>>,
}

impl FileIter {
    pub(crate) fn new(client: Client, chat: Chat) -> Self {
        let message_iter = client.iter_messages(chat);
        let next_message = Box::pin(Self::next_message(message_iter));

        Self { next_message }
    }

    async fn next_message(
        mut message_iter: MessageIter,
    ) -> (Result<Option<Message>, InvocationError>, MessageIter) {
        (message_iter.next().await, message_iter)
    }
}

impl Stream for FileIter {
    type Item = File;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.next_message.as_mut().poll(cx) {
            Poll::Ready(r) => {
                self.next_message = Box::pin(Self::next_message(r.1));
                match r.0 {
                    Ok(Some(m)) => {
                        if let Ok(file_metadata) = serde_json::from_str::<FileMetadata>(m.text()) {
                            return Poll::Ready(Some(File::new(file_metadata, m.id())));
                        }
                        cx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                    Ok(None) => Poll::Ready(None),
                    Err(_) => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
