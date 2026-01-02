use crate::domain::errors::FatalError;

pub enum SystemEvent {
    TaskFatalError {
        task_name: String,
        error: FatalError,
    },
    TaskCompleted {
        task_name: String,
    },
    ShutdownTriggered,
}
