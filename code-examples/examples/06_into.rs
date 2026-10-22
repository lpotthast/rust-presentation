#[allow(dead_code)] // the fields are only printed via Debug
#[derive(Debug)]
struct User {
    name: String,
    admin: bool,
}

impl User {
    // variant 1: fn new(name: &str) -> Self { Self { name: String::from(name), admin: false } }
    //            User::new(&input) copies the text
    // variant 2: fn new(name: String) -> Self { Self { name, admin: false } }
    //            User::new(String::from("Kevin")) at every call with a literal
    fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), admin: false }
    }
}

// same, generic syntax
fn new_user<S: Into<String>>(name: S) -> User {
    User { name: name.into(), admin: false }
}

fn main() {
    let kevin = User::new("Kevin"); // &str is converted
    let input = String::from("Stuart"); // e.g. from a request
    let stuart = User::new(input); // moved, no copy
    let bob = new_user('B'); // String also implements From<char>

    println!("{kevin:?}"); // User { name: "Kevin", admin: false }
    println!("{stuart:?}"); // User { name: "Stuart", admin: false }
    println!("{bob:?}"); // User { name: "B", admin: false }
}
