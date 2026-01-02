use std::pin::Pin;

use tokio_util::sync::CancellationToken;

#[derive(thiserror::Error, Debug, Clone)]
#[error("Fatal system error: {0}")]
pub struct FatalError(pub String);

#[derive(thiserror::Error, Debug, Clone)]
#[error("Recoverable task error: {0}")]
pub struct RecoverableError(pub String);

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Fatal error: {error}")]
    Fatal {
        #[from]
        error: FatalError,
    },

    #[error("Recoverable error: {error}")]
    Recoverable {
        #[from]
        error: RecoverableError,
    },
}

pub enum SystemEvent {
    TaskFatalError {
        task_name: String,
        error: FatalError,
    },
    ShutdownTriggered,
}

pub type TaskResult = Result<(), AppError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn Fn(CancellationToken) -> BoxedFuture + Send>;
