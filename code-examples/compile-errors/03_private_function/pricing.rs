// src/pricing.rs
pub fn order_total(price: u32, quantity: u32) -> u32 {
    let subtotal = price * quantity;
    subtotal + shipping_cost(subtotal)
}

fn shipping_cost(subtotal: u32) -> u32 {
    if subtotal >= 1000 { 0 } else { 495 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn free_shipping_from_10_euros() {
        assert_eq!(shipping_cost(1000), 0);
    }
}
