use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use crate::runtime::Task;

// TODO:
// 1. way to queue tasks on it
// 2. way to park the thread (the worker encapsulates the underlying thread)
// 3. way to get load
pub struct Worker {
    // XXX: this can probably be optimized
    tasks: Mutex<VecDeque<Arc<Task>>>,
    thread_handle: Mutex<Option<JoinHandle<()>>>,
}

impl Worker {
    pub fn new() -> Arc<Self> {
        let me = Arc::new(Self {
            tasks: Mutex::new(VecDeque::new()),
            thread_handle: Mutex::new(None),
        });

        *me.thread_handle.lock().expect("lock poisoned") = Some(std::thread::spawn({
            let me = Arc::clone(&me);
            move || me.run()
        }));
        me
    }

    pub fn submit(&self, task: Arc<Task>) {
        let mut tasks = self.tasks.lock().expect("lock poisoned");
        tasks.push_front(task);
        // TODO: unpark
    }

    fn run(self: Arc<Self>) {
        // brunoroy: change loop for while
        loop {
            if let Some(task) = self.tasks.lock().expect("lock poisoned").pop_front() {
                task.poll();
            } else {
                // nothing to do, park thread
                // std::thread::park()
                std::thread::sleep(std::time::Duration::from_millis(10))
            }
        }
    }
}
