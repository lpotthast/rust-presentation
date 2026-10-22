#[allow(unused_variables)] fn main() -> Result<(), std::num::ParseIntError> {
    let input = "80a"; // e.g. read from a config file

    let port: u16 = match input.parse() {
        Ok(port) => port,
        Err(_) => 8080,
    };
    let port: u16 = input.parse().unwrap_or(8080);
    let port: u16 = input.parse().expect("port must be a number");
    let port: u16 = input.parse()?;
    println!("{port}");
    Ok(())
}

// slide "Handling a Result": the body of main is the code on the slide.
// fn main stays on line 1, so the panic location (line 9, column 35) matches the slide.
