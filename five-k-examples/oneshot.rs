use std::time::Duration;

use five_k::{channels::oneshot, runtime::Runtime, timer::Timer};

fn main() {
    let mut rt = Runtime::new();

    let (tx, rx) = oneshot::channel();

    rt.spawn(async move {
        // wait 3 seconds then send something on the oneshot
        let timer = Timer::new(Duration::from_secs(3));
        timer.await;
        tx.send(3);
    });

    rt.block_on(async {
        println!("Waiting on the oneshot");
        let value = rx.await;
        println!("Received the value: {:?}", value);
    });
}
