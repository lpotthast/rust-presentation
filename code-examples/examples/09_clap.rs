use clap::Parser;

/// Reorders a product from the supplier
#[derive(Parser)]
struct Args {
    /// Product number, e.g. BANANA-01
    sku: String,

    /// Number of boxes to order
    #[arg(short, long, default_value_t = 1)]
    quantity: u8,
}

fn main() {
    let args = Args::parse();
    println!("Ordering {} x {}", args.quantity, args.sku);
}
