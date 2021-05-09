use std::time::Duration;
use tokio::{join, time::sleep};

#[tokio::main]
async fn main() {
    let future1 = async move {
        println!("Started future1");
        sleep(Duration::from_secs(5)).await;
        println!("Finished future1");
    };

    let future2 = async move {
        println!("Started future2");
        sleep(Duration::from_secs(3)).await;
        println!("Finished future2");
    };

    join!(future1, future2);
}
