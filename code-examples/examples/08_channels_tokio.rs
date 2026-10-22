use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // tx: sender, rx: receiver

    for customer in ["Kevin", "Stuart"] {
        let tx = tx.clone();
        tokio::spawn(async move {
            for id in 1..=2 {
                let order = format!("order {id} from {customer}");
                tx.send(order).await.unwrap();
            }
        });
    }
    drop(tx); // the channel closes when all senders are gone

    while let Some(order) = rx.recv().await {
        println!("processing {order}");
    }
}
