// all three cases panic, choose one with an argument:
// cargo run --example 07_panic -- index | unwrap | expect
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("index") => {
            #[allow(clippy::useless_vec)] // an array with a constant index would already fail to compile
            let sizes = vec!["S", "M", "L"];
            let size = sizes[5]; // index out of bounds: the len is 3 but the index is 5
            println!("{size}");
        }
        Some("unwrap") => {
            let port: u16 = "80a".parse().unwrap(); // called `Result::unwrap()` on an `Err` value
            println!("{port}");
        }
        _ => {
            let port: u16 = "80a".parse().expect("port must be a number");
            println!("{port}");
        }
    }
}
