use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct DummyWaker;

impl Wake for DummyWaker {
    fn wake(self: Arc<Self>) {
        // do nothing here as we just need to wake the task
    }
}

pub fn dummy_block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    let waker: Waker = Waker::from(Arc::new(DummyWaker));

    let mut cx = Context::from_waker(&waker);

    let mut pinned_future = Box::pin(future);

    loop {
        match pinned_future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                // avoid 100% CPU usage
                std::thread::yield_now();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_block() {
        let result = dummy_block_on(async {
            // Simulate some asynchronous operation
            std::thread::sleep(std::time::Duration::from_millis(100));
            42
        });
        assert_eq!(result, 42);
    }
}
