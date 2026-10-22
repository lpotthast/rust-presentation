use std::collections::HashMap;

fn total_price(price: u32, quantity: u32) -> u32 {
    price * quantity // no semicolon: this is the return value
}

fn main() {
    println!("Hello, world!");

    // variables
    let max_items = 10;       // inferred: i32
    // max_items = 20;        // error: immutable
    let mut cart_items = 0;   // mutable
    cart_items += 1;
    let price: u32 = 450;     // explicit, otherwise i32
    let input = "  Banana  ";
    let input = input.trim(); // shadowing
    println!("{max_items} {cart_items} {price} {input}");

    // tuple
    let item: (&str, u32) = ("Banana", 450);
    let (name, price) = item;
    println!("{name} {price}");

    // array: fixed length
    let sizes: [&str; 3] = ["S", "M", "L"];

    // Vec: growable (heap)
    let mut cart = vec![450, 1200];
    cart.push(299);

    // HashMap
    let mut stock = HashMap::new();
    stock.insert("Banana", 12);
    println!("{sizes:?} {cart:?} {stock:?}");

    // functions and expressions (prices in cents)
    let total = total_price(450, 3);
    let shipping = if total >= 1000 { 0 } else { 495 };
    println!("total: {total}, shipping: {shipping}"); // total: 1350, shipping: 0

    // range: 0 to 4
    for i in 0..5 {
        println!("{i}");
    }

    let cart = ["Banana", "Goggles", "Overalls"];

    // iterate over a collection
    for item in cart {
        println!("{item}");
    }

    // with index: 1. Banana, 2. Goggles, ...
    for (i, item) in cart.iter().enumerate() {
        println!("{}. {item}", i + 1);
    }

    let mut stock = 3;

    // while: repeats as long as the condition holds
    while stock > 0 {
        println!("sold one, {stock} left");
        stock -= 1;
    }

    // loop: runs until break, break can return a value
    let mut attempt = 0;
    let delay_ms = loop {
        attempt += 1;
        if attempt == 3 {
            break attempt * 100; // retry delay
        }
    };
    println!("{delay_ms}");
}
