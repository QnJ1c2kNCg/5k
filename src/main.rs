use crate::runtime::Runtime;

mod runtime;

fn main() {
    print!("Hello, ");

    let my_future = async {
        println!("world");
    };

    let mut rt = Runtime::new();
    rt.spawn(Box::pin(my_future));

    rt.start();
}
