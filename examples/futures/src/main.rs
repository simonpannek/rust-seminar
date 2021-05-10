use std::time::Duration;
use tokio::{join, time::sleep};

async fn async_function() {
    println!("Started task1");
    sleep(Duration::from_secs(5)).await;
    println!("Finished task1");
}

#[tokio::main]
async fn main() {
    let future1 = async_function();

    let future2 = async {
        println!("Started task2");
        sleep(Duration::from_secs(3)).await;
        println!("Finished task2");
    };

    // Join both futures
    join!(future1, future2);
}
