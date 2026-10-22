use std::thread;
fn main() {
    let mut sold = Vec::new();
    thread::scope(|s| {
        s.spawn(|| sold.push("Banana"));
        s.spawn(|| sold.push("Goggles"));
    });
}
