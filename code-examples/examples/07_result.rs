use std::error::Error;
use std::fs;

// without ?: every error is handled with match and an early return
fn read_port_match(path: &str) -> Result<u16, Box<dyn Error>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };
    let port: u16 = match content.trim().parse() {
        Ok(port) => port,
        Err(error) => return Err(error.into()),
    };
    Ok(port)
}

// the same function with ?
fn read_port(path: &str) -> Result<u16, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    Ok(port)
}

fn main() {
    match fs::read_to_string("port.txt") {
        Ok(content) => println!("Port: {content}"),
        Err(error) => eprintln!("Could not read file: {error}"),
    }

    let n: i32 = "42".parse().unwrap();                  // panics on error
    let m: i32 = "7".parse().expect("must be a number"); // panics with a custom message
    println!("{n} {m}");

    for result in [read_port_match("port.txt"), read_port("port.txt")] {
        match result {
            Ok(port) => println!("Port: {port}"),
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
