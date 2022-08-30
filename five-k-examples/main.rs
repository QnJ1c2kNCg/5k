use std::{
    io::{self, Write},
    time::Duration,
};

use five_k::{runtime::Runtime, timer::Timer};

fn main() {
    let rt = Runtime::new();
    rt.block_on(inner_main());
}

async fn inner_main() {
    print!("Hello, ");
    io::stdout().flush().unwrap();

    let my_future = async {
        let timer = Timer::new(Duration::from_secs(4));
        timer.await;
        println!("world");
    };

    my_future.await;

    five_k::pending::Pending::default().await
}
