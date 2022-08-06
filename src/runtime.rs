use std::{
    future::Future,
    pin::Pin,
    ptr,
    sync::{mpsc, Arc, Mutex},
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
    scheduled_tasks: mpsc::Receiver<Arc<Task>>,
    sender: mpsc::Sender<Arc<Task>>,
    waker: Waker,
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Task {
    fn poll(self: Arc<Self>, cx: &mut Context) {
        let mut future = self.future.lock().expect("lock poisoned");
        future.as_mut().poll(cx);
    }
}

impl Runtime {
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel();

        let waker = unsafe {
            let raw_waker = RawWaker::new(ptr::null(), &V_TABLE);
            Waker::from_raw(raw_waker)
        };

        Self {
            scheduled_tasks: recv,
            sender: send,
            waker,
        }
    }

    pub fn spawn(&mut self, f: Pin<Box<dyn Future<Output = ()>>>) {
        // create task
        let task = Arc::new(Task {
            future: Mutex::new(f),
        });

        // schedule the task
        self.sender.send(task);
    }

    pub fn run(&mut self) {
        let mut cx = Context::from_waker(&self.waker);
        while let Ok(task) = self.scheduled_tasks.recv() {
            task.poll(&mut cx);
        }
    }
}
