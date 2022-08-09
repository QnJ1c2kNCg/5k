use std::{
    io::{self, Write},
    time::Duration,
};

use five_k::{runtime::Runtime, timer::Timer};

fn main() {
    print!("Hello, ");
    io::stdout().flush().unwrap();

    let my_future = async {
        let timer = Timer::new(Duration::from_secs(4));
        timer.await;
        println!("world");
    };

    let mut rt = Runtime::new();
    rt.spawn(my_future);

    rt.run();
    rt.block_on(async { five_k::pending::Pending::default().await })
}
