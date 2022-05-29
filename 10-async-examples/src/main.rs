use futures::future::join_all;
use tokio::time::{sleep, Duration};

async fn hello_from(runner: i32) {
    println!("[{}] Hello world!", runner);

    sleep(Duration::from_millis(1_000)).await;

    println!("[{}] Goodbye world!", runner);
}

#[tokio::main]
async fn main() {
    let futures = (1..=10).map(hello_from);

    join_all(futures).await;
}
