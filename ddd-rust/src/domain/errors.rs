#[derive(thiserror::Error, Debug, Clone)]
#[error("Fatal system error: {0}")]
pub struct FatalError(pub String);

impl From<&str> for FatalError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for FatalError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(thiserror::Error, Debug, Clone)]
#[error("Recoverable task error: {0}")]
pub struct RecoverableError(pub String);

impl From<&str> for RecoverableError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for RecoverableError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Fatal error: {error}")]
    Fatal { error: FatalError },

    #[error("Recoverable error: {error}")]
    Recoverable { error: RecoverableError },
}

impl From<FatalError> for AppError {
    fn from(error: FatalError) -> Self {
        Self::Fatal { error }
    }
}

impl From<RecoverableError> for AppError {
    fn from(error: RecoverableError) -> Self {
        Self::Recoverable { error }
    }
}
