use std::pin::Pin;

use tokio_util::sync::CancellationToken;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Fatal system error: {0}")]
    Fatal(String),

    #[error("Recoverable task error: {0}")]
    Recoverable(String),
}

pub enum SystemEvent {
    TaskFatalError { task_name: String, error: AppError },
    TaskRecovered { task_name: String },
    ShutdownTriggered,
}

pub type TaskResult = Result<(), AppError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn Fn(CancellationToken) -> BoxedFuture + Send>;
