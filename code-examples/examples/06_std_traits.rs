use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Money(u32); // amount in cents

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02} EUR", self.0 / 100, self.0 % 100)
    }
}

fn main() {
    let price = Money(1649);
    println!("{price}");   // 16.49 EUR
    println!("{price:?}"); // Money(1649)

    let copy = price; // Copy: price stays valid
    assert_eq!(copy, price);
}
