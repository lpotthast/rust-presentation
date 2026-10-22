struct Money(u32); // amount in cents

fn total_price(price: Money, quantity: u32) -> Money {
    Money(price.0 * quantity)
}

fn main() {
    let total = total_price(Money(450), 3);
    println!("{} cents", total.0); // 1350 cents

    // memory layout is the same as a plain u32
    assert_eq!(size_of::<Money>(), size_of::<u32>());
}
