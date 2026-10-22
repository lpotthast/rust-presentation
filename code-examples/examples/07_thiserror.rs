use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read file")]
    Io(#[from] std::io::Error),

    #[error("invalid port")]
    Parse(#[from] std::num::ParseIntError),

    #[error("port {0} is reserved")]
    Reserved(u16),
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    if port < 1024 {
        return Err(ConfigError::Reserved(port));
    }
    Ok(port)
}

fn main() -> anyhow::Result<()> {
    // the caller can react to individual errors
    match read_port("port.txt") {
        Ok(port) => println!("port {port}"),
        Err(ConfigError::Io(_)) => println!("default 8080"),
        Err(error) => eprintln!("{error}"),
    }

    // or pass everything on with anyhow, which also prints the cause
    let port = read_port("port.txt")?;
    println!("port {port}");
    Ok(())
}
