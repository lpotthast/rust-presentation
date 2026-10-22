macro_rules! check {
    ($cond:expr) => {
        if !$cond {
            println!("check failed: {}", stringify!($cond));
        }
    };
}

fn main() {
    let port = 80;
    check!(port >= 1024); // check failed: port >= 1024
    let v = vec![1, 2, 3]; // vec! is a macro
    println!("{v:?}");     // so is println!
}
