use std::{
    io::{self, Write},
    time::Duration,
};

use crate::{runtime::Runtime, timer::Timer};

mod runtime;
mod timer;

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
    loop {}
}
