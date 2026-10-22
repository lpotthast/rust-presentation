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
