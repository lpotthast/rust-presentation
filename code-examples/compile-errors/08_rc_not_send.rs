use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

async fn process_order() {
    let order_id = Rc::new(1001); // Rc is not Send
    sleep(Duration::from_millis(10)).await;
    println!("order {order_id}");
}

#[tokio::main]
async fn main() {
    tokio::spawn(process_order()).await.unwrap();
}
