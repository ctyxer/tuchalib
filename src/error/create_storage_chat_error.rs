#[derive(Debug, Clone)]
pub enum CreateStorageChatError {
    ChatIsNotCreated,
    CreatedStorageChatIsNotFound,
}

impl std::error::Error for CreateStorageChatError {}

impl std::fmt::Display for CreateStorageChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateStorageChatError::ChatIsNotCreated => write!(f, "Chat is not created."),
            CreateStorageChatError::CreatedStorageChatIsNotFound => write!(f, "Created storage chat is not found."),
        }
    }
}