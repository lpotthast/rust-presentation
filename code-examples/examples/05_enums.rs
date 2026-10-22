// only affects other crates: they must add a `_` arm when matching
#[non_exhaustive]
pub enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}

// should a failed call to the payment provider be retried?
fn should_retry(http_status: u16, attempt: u32) -> bool {
    match http_status {
        200..=299 => false,               // ranges
        429 => attempt < 5,               // single values
        408 | 503 if attempt < 3 => true, // alternatives with a guard
        _ => false,                       // everything else
    }
}

#[derive(Debug)]
struct User {
    name: String,
    admin: bool,
}

impl User {
    fn new(name: &str) -> Self {
        Self { name: String::from(name), admin: false }
    }

    fn greeting(&self) -> String {
        format!("Hello {}", self.name)
    }
}

fn find_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User::new("Kevin"))
    } else {
        None
    }
}

fn reserve(quantity: u32, in_stock: u32) -> Result<u32, String> {
    if quantity > in_stock {
        Err(format!("only {in_stock} in stock"))
    } else {
        Ok(in_stock - quantity)
    }
}

fn main() {
    // creating values
    let pending = OrderStatus::Pending;
    let shipped = OrderStatus::Shipped {
        tracking_id: String::from("DE123"),
    };
    let cancelled = OrderStatus::Cancelled { reason: String::from("out of stock") };

    println!("{}", describe(&shipped)); // shipped, tracking ID DE123
    println!("{}", describe(&pending));
    println!("{}", describe(&cancelled));

    println!("{}", should_retry(503, 1)); // true

    // Option
    match find_user(42) {
        Some(user) => println!("Found: {}", user.name),
        None => println!("No user"),
    }

    if let Some(user) = find_user(1) {
        println!("{}", user.greeting());
    }

    let user = find_user(7).unwrap_or(User::new("Guest"));
    println!("{user:?}");

    // let ... else
    let Some(kevin) = find_user(1) else {
        return;
    };
    println!("{} is admin: {}", kevin.name, kevin.admin);

    // Result
    match reserve(3, 2) {
        Ok(left) => println!("Reserved, {left} left"),
        Err(message) => println!("Error: {message}"),
    } // Error: only 2 in stock

    if let Ok(left) = reserve(1, 10) {
        println!("Reserved, {left} left");
    } // Reserved, 9 left
}
