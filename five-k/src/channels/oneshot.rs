use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

pub struct Sender<T> {
    value: Arc<Mutex<Option<T>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl<T> Sender<T> {
    // fn new() -> Self {
    //     Self {
    //         value: Arc::new(Mutex::new(None)),
    //     }
    // }

    pub fn send(self, val: T) {
        // TODO: should probably just move all of this in a state struct
        let mut value = self.value.lock().expect("lock poisoned");
        let mut waker = self.waker.lock().expect("lock poisoned");

        *value = Some(val);
        // XXX: What happens if we send before polling??
        if let Some(waker) = waker.take() {
            waker.wake();
        }
    }
}

pub struct Receiver<T> {
    value: Arc<Mutex<Option<T>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

// impl<T> Receiver<T> {
//     fn new() -> Self {
//         Self {
//             value: Arc::new(Mutex::new(None)),
//         }
//     }
// }

impl<T> Future for Receiver<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut value = self.value.lock().expect("lock poisoned");
        if value.is_some() {
            Poll::Ready(value.take().unwrap())
        } else {
            let mut waker = self.waker.lock().expect("lock poisoned");
            *waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let value = Arc::new(Mutex::new(None));
    let waker = Arc::new(Mutex::new(None));
    (
        Sender {
            value: Arc::clone(&value),
            waker: Arc::clone(&waker),
        },
        Receiver { value, waker },
    )
}
