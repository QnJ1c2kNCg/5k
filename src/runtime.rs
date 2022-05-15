use std::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

unsafe fn clone(_: *const ()) -> RawWaker {
    todo!()
}
unsafe fn wake(_: *const ()) {
    todo!()
}
unsafe fn wake_by_ref(_: *const ()) {
    todo!()
}
unsafe fn drop(data: *const ()) {
    assert!(data.is_null())
}
static V_TABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

pub struct Runtime {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
    pub fn spawn(&mut self, f: Pin<Box<dyn Future<Output = ()>>>) {
        self.tasks.push(f);
    }

    pub fn start(&mut self) {
        let waker = unsafe {
            let raw_waker = RawWaker::new(ptr::null(), &V_TABLE);
            Waker::from_raw(raw_waker)
        };
        let mut cx = Context::from_waker(&waker);

        for task in &mut self.tasks {
            if let Poll::Ready(_) = task.as_mut().poll(&mut cx) {
                break;
            }
        }
    }
}
