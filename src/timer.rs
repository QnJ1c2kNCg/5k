use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Poll, Waker},
    thread::{self, sleep},
    time::Duration,
};

pub struct Timer {
    state: Arc<Mutex<TimerState>>,
}

struct TimerState {
    is_completed: bool,
    waker: Option<Waker>,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            is_completed: false,
            waker: None,
        }));
        {
            let state = Arc::clone(&state);

            thread::spawn(move || {
                sleep(duration);
                let mut state = state.lock().expect("lock poisoned");
                state.is_completed = true;
                if let Some(waker) = state.waker.take() {
                    waker.wake();
                }
            });
        }

        Self { state }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.state.lock().expect("lock poisoned");
        if state.is_completed {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
