use std::path::{Path, PathBuf};

fn print_label(text: &str) {
    println!("Label: {text}");
}

fn total(prices: &[u32]) -> u32 {
    prices.iter().sum() // iterators: chapter 6
}

fn export(path: &Path) {
    println!("Exporting to {}", path.display());
}

fn main() {
    // slide "String and &str"
    let address = String::from("Banana Street 1"); // owns the text on the heap
    let street: &str = &address[0..6]; // string slice: "Banana"
    let city = "Minion Town"; // literal: &'static str
    println!("{street}, {city}, {} bytes", address.len()); // Banana, Minion Town, 15 bytes

    let product = "Müsli";
    println!("{} bytes, {} characters", product.len(), product.chars().count()); // 6 bytes, 5 characters
    // address[0] does not compile: error[E0277]: the type `str` cannot be indexed by `{integer}`

    // slide "Owned & Borrowed Types"
    let label: String = String::from("A-1001");
    print_label(&label); // &String -> &str
    print_label("B-2002"); // string literal, also a &str

    let cart: Vec<u32> = vec![450, 1299];
    println!("{}", total(&cart)); // &Vec<u32> -> &[u32], 1749
    println!("{}", total(&[450, 1299])); // array, also a &[u32], 1749

    // same pattern for paths (notes only)
    let path: PathBuf = PathBuf::from("/srv/orders.csv");
    export(&path); // &PathBuf -> &Path
    export(Path::new("orders.csv")); // &Path directly
}
