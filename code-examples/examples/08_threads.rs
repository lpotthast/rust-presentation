use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let orders = vec![vec![450, 1200], vec![299], vec![1299, 450, 99]];
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for order in orders {
        let total = Arc::clone(&total);
        handles.push(thread::spawn(move || {
            let order_total: u32 = order.iter().sum();
            *total.lock().unwrap() += order_total;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Total: {} cents", *total.lock().unwrap()); // 3797
}
