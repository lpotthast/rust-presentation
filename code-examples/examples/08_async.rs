use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch_user(id: u32) -> String {
    // simulates a database query
    sleep(Duration::from_millis(100)).await;
    format!("user {id}")
}

async fn fetch_orders(user_id: u32) -> Vec<u32> {
    // simulates an HTTP request
    sleep(Duration::from_millis(100)).await;
    vec![user_id * 10, user_id * 10 + 1]
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // sequential
    let user = fetch_user(1).await;

    // separate task, starts running in the background
    let handle = tokio::spawn(fetch_orders(1));

    // concurrent: both wait at the same time
    let (other_user, other_orders) =
        tokio::join!(fetch_user(2), fetch_orders(2));

    // wait for the result of the task
    let orders = handle.await.unwrap();

    println!("{user}: {orders:?}");
    println!("{other_user}: {other_orders:?}");
    // user 1: [10, 11]
    // user 2: [20, 21]
    println!("took {} ms", start.elapsed().as_millis()); // about 200 ms
}
