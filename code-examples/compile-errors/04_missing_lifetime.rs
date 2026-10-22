fn host_or_default(configured: &str, default: &str) -> &str {
    if configured.is_empty() {
        default
    } else {
        configured
    }
}

fn main() {
    println!("{}", host_or_default("", "localhost"));
}
