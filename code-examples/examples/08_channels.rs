use std::sync::mpsc;
use std::thread;
fn main() {
    let orders = vec![
        vec![450, 1200], // prices in cents
        vec![299],
        vec![1299, 450, 99],
    ];
    let (tx, rx) = mpsc::channel(); // tx: sender, rx: receiver
    for order in orders {
        let tx = tx.clone();
        thread::spawn(move || {
            let order_total: u32 = order.iter().sum();
            tx.send(order_total).unwrap();
        });
    }
    drop(tx); // the channel closes when all senders are gone

    let total: u32 = rx.iter().sum();
    println!("Total: {total} cents"); // 3797
}
