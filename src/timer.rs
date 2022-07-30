use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Poll, Waker},
    thread::{self, sleep},
    time::Duration,
};

pub struct Timer {
    is_completed: Arc<AtomicBool>,
    duration: Duration,
    waker: Option<Waker>,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            is_completed: Arc::new(AtomicBool::new(false)),
            duration,
            waker: None,
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.is_completed.load(Ordering::Relaxed) {
            Poll::Ready(())
        } else {
            {
                let waker = cx.waker().clone();
                let is_completed = Arc::clone(&self.is_completed);
                let duration = self.duration;
                thread::spawn(move || {
                    sleep(duration);
                    is_completed.store(true, Ordering::Relaxed);
                    waker.wake();
                });
            }
            Poll::Pending
        }
    }
}
