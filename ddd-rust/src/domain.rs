pub mod args;
pub mod errors;
pub mod events;

use std::{pin::Pin, sync::Arc};

use crossbeam_channel::Sender;
use tokio_util::sync::CancellationToken;

pub type TaskResult = TaskResultGeneric<()>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn Fn(CancellationToken) -> BoxedFuture + Send>;
pub struct TaskDefinition {
    pub name: Arc<str>,
    pub factory: TaskFactory,
}

pub type TaskResultGeneric<T> = Result<T, errors::AppError>;

pub fn build_task<F, Fut, C, R>(
    f: F,
    ct: CancellationToken,
    tx: Sender<C>,
) -> impl Future<Output = TaskResultGeneric<R>> + Send + 'static
where
    F: FnOnce(CancellationToken, Sender<C>) -> Fut,
    Fut: Future<Output = TaskResultGeneric<R>> + Send + 'static,
{
    f(ct, tx)
}

pub fn build_task_with_name<F, Fut, C, R>(
    name: Arc<str>,
    f: F,
    ct: CancellationToken,
    tx: Sender<C>,
) -> (
    Arc<str>,
    impl Future<Output = TaskResultGeneric<R>> + Send + 'static,
)
where
    F: FnOnce(CancellationToken, Sender<C>) -> Fut,
    Fut: Future<Output = TaskResultGeneric<R>> + Send + 'static,
{
    (name, build_task(f, ct, tx))
}
