use std::{
    future::Future,
    pin::Pin,
    sync::{mpsc, Arc, Mutex},
    task::{Context, RawWaker, RawWakerVTable, Waker},
    thread::{self},
};

use crate::worker::Worker;

// brunoroy: why did they use a vtable instead of just a trait?

unsafe fn clone(data_ptr: *const ()) -> RawWaker {
    RawWaker::new(data_ptr, &V_TABLE)
}

unsafe fn wake(data_ptr: *const ()) {
    let vtable_data: &VTableData = &*(data_ptr as *const VTableData);
    vtable_data
        .task
        .sender
        .lock()
        .unwrap()
        .send(Arc::clone(&vtable_data.task))
        .expect("we know the receiver hasn't been dropped");
}

unsafe fn wake_by_ref(data_ptr: *const ()) {
    // no idea if that is right
    wake(data_ptr)
}

unsafe fn drop(_: *const ()) {
    // leaking!
}

static V_TABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

struct VTableData {
    task: Arc<Task>,
}

pub struct Runtime {
    sender: mpsc::Sender<Arc<Task>>,
}

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    // i don't think this needs the mutex, but we want Task to be sync for now
    sender: Mutex<mpsc::Sender<Arc<Task>>>,
}

impl Task {
    // brunoroy: do i need this arc here?
    pub fn poll(self: Arc<Self>) {
        let vtable_data = Box::new(VTableData {
            task: Arc::clone(&self),
        });
        let vtable_data: *const () = Box::into_raw(vtable_data) as *const ();
        let waker = unsafe {
            let raw_waker = RawWaker::new(vtable_data, &V_TABLE);
            Waker::from_raw(raw_waker)
        };
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.lock().expect("lock poisoned");

        // we don't care if it returns Ready or Pending, the task will schedule
        // itself if it is pending
        let _ = future.as_mut().poll(&mut cx);
    }
}

impl Runtime {
    pub fn new() -> Self {
        let (sender, recv) = mpsc::channel::<Arc<Task>>();

        // TODO: store join handle for cleanup
        thread::spawn(move || {
            let worker = Worker::new();
            while let Ok(task) = recv.recv() {
                // task.poll();
                worker.submit(task)
            }
        });

        Self { sender }
    }

    pub fn spawn(&mut self, f: impl Future<Output = ()> + 'static + Send) {
        // create task
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(f)),
            sender: Mutex::new(self.sender.clone()),
        });

        // schedule the task
        self.sender
            .send(task)
            .expect("we know the receiver hasn't been dropped");
    }

    pub fn block_on(&self, f: impl Future<Output = ()> + 'static + Send) {
        let (sender, recv) = mpsc::channel::<Arc<Task>>();
        // create task
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(f)),
            sender: Mutex::new(sender),
        });

        task.poll();

        while let Ok(task) = recv.recv() {
            task.poll();
        }
    }
}
