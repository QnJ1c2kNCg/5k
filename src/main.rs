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
        let timer = Timer::new(Duration::from_secs(5));
        timer.await;
        println!("world");
    };

    let mut rt = Runtime::new();
    rt.spawn(Box::pin(my_future));

    rt.start();
}
