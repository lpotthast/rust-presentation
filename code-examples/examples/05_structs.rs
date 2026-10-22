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

    fn promote(&mut self) {
        self.admin = true;
    }
}

fn main() {
    let mut kevin = User::new("Kevin");
    kevin.promote();
    println!("{}", kevin.greeting()); // Hello Kevin
    println!("{kevin:?}");            // User { name: "Kevin", admin: true }
}
