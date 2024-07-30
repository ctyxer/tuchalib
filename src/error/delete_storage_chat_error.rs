#[derive(Debug, Clone)]
pub enum DeleteStorageChatError {
    CannotDeleteChat,
}

impl std::error::Error for DeleteStorageChatError {}

impl std::fmt::Display for DeleteStorageChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteStorageChatError::CannotDeleteChat => write!(f, "Cannot delete chat."),
        }
    }
}
