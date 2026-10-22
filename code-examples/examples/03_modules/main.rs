// src/main.rs
mod pricing; // includes src/pricing.rs

fn main() {
    let total = pricing::order_total(450, 3);
    // pricing::shipping_cost(total);
    // error: function is private
    println!("{total}");
}
