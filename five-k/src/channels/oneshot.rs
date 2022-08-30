use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

struct OneShotState<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

pub struct Sender<T> {
    state: Arc<Mutex<OneShotState<T>>>,
}

impl<T> Sender<T> {
    pub fn send(self, val: T) {
        let mut state = self.state.lock().expect("lock poisoned");

        state.value = Some(val);
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

pub struct Receiver<T> {
    state: Arc<Mutex<OneShotState<T>>>,
}

impl<T> Future for Receiver<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.state.lock().expect("lock poisoned");
        if state.value.is_some() {
            Poll::Ready(state.value.take().unwrap())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let state = Arc::new(Mutex::new(OneShotState {
        value: None,
        waker: None,
    }));

    (
        Sender {
            state: Arc::clone(&state),
        },
        Receiver { state },
    )
}
