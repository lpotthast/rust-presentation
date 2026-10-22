use std::thread;

fn main() {
    let orders = vec![
        vec![450, 1200], // prices in cents
        vec![299],
        vec![1299, 450, 99],
    ];
    let mut handles = vec![];

    for order in orders {
        handles.push(thread::spawn(move || {
            let order_total: u32 = order.iter().sum();
            order_total
        }));
    }

    let mut total = 0;
    for h in handles {
        total += h.join().unwrap();
    }
    println!("Total: {total} cents"); // 3797
}
