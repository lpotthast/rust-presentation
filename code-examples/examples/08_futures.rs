use std::time::Duration;
use tokio::time::sleep;

// long form: a normal function that returns a future
#[allow(clippy::manual_async_fn)] // on purpose, clippy suggests async fn
fn fetch_user(id: u32) -> impl Future<Output = String> {
    async move {
        // simulates a database query
        sleep(Duration::from_millis(100)).await;
        format!("user {id}")
    }
}

// the same function as async fn (syntactic sugar)
async fn fetch_user_sugar(id: u32) -> String {
    // simulates a database query
    sleep(Duration::from_millis(100)).await;
    format!("user {id}")
}

#[tokio::main]
async fn main() {
    let future = fetch_user(1); // nothing runs yet
    let user = future.await;    // now it runs
    println!("{user}");

    println!("{}", fetch_user_sugar(2).await);
}
