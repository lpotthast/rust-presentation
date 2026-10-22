fn length(text: &String) -> usize { // borrows for reading
    text.len()
}

fn append(text: &mut String) { // borrows for writing
    text.push_str(", Minion Town");
}

fn host_or_default<'a>(configured: &'a str, default: &'a str) -> &'a str {
    if configured.is_empty() {
        default
    } else {
        configured
    }
}

fn main() {
    let mut address = String::from("Banana Street 1");
    append(&mut address);
    let n = length(&address);
    println!("{address} ({n})"); // Banana Street 1, Minion Town (28)

    // Borrowing Rules: the borrow held by first ends at its last use
    let mut cart = vec!["Banana", "Goggles"];
    let first = &cart[0];
    println!("{first}");
    cart.push("Overalls");

    let configured = String::new();
    println!("{}", host_or_default(&configured, "localhost")); // localhost
}
