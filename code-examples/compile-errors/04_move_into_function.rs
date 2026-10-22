fn length(text: String) -> usize { // takes ownership
    text.len()
}

fn main() {
    let address = String::from("Banana Street 1");
    let n = length(address);
    println!("{address} ({n})"); // error: address was moved
}
