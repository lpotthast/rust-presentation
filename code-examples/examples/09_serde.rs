use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    #[serde(default)]
    admin: bool,
}

fn main() {
    let json = r#"{ "id": 1, "name": "Kevin" }"#;
    let user: User = serde_json::from_str(json).unwrap();
    println!("{user:?}");

    println!("{}", serde_json::to_string_pretty(&user).unwrap());
}
