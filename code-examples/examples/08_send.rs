use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// solution 1: Arc instead of Rc
async fn process_order() {
    // with Rc::new(1001), tokio::spawn does not compile because Rc is not Send
    let order_id = Arc::new(1001);
    sleep(Duration::from_millis(10)).await;
    println!("order {order_id}");
}

// solution 2: the Rc lives in a block that ends before the .await
// (an explicit drop(order_id) before the .await is not enough)
async fn process_order_scoped() {
    {
        let order_id = Rc::new(1001);
        println!("order {order_id}");
    }
    sleep(Duration::from_millis(10)).await;
}

#[tokio::main]
async fn main() {
    tokio::spawn(process_order()).await.unwrap();
    tokio::spawn(process_order_scoped()).await.unwrap();
}
