use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ReservedPort(u16);

impl fmt::Display for ReservedPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "port {} is reserved", self.0)
    }
}

impl Error for ReservedPort {} // source() keeps its default: no cause

fn main() {
    // Box<dyn Error> holds any type that implements Error
    let errors: Vec<Box<dyn Error>> = vec![
        Box::new(ReservedPort(80)),
        Box::new("80a".parse::<u16>().unwrap_err()),
        Box::new(std::fs::read_to_string("missing.txt").unwrap_err()),
    ];
    for error in &errors {
        println!("{error}"); // Display
    }
    println!("{:?}", errors[0]); // Debug: ReservedPort(80)
    println!("{}", errors[0].source().is_none()); // true
}
