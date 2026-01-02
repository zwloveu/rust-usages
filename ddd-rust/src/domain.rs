pub mod args;
pub mod errors;
pub mod events;

use std::pin::Pin;

use tokio_util::sync::CancellationToken;

pub type TaskResult = Result<(), errors::AppError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn Fn(CancellationToken) -> BoxedFuture + Send>;
