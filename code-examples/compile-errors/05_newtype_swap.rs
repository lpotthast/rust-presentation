struct Money(u32); // amount in cents

fn total_price(price: Money, quantity: u32) -> Money {
    Money(price.0 * quantity)
}

fn main() {
    let total = total_price(3, Money(450));
    println!("{} cents", total.0);
}
