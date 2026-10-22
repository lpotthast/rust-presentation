use rootcause::prelude::*;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("cannot read file")]
    Io,
    #[error("invalid port")]
    Parse,
    #[error("port {0} is reserved")]
    Reserved(u16),
}

// typed: callers can match on ConfigError
fn read_port(path: &str) -> Result<u16, Report<ConfigError>> {
    let content = fs::read_to_string(path).context(ConfigError::Io)?;
    let port: u16 = content.trim().parse().context(ConfigError::Parse)?;
    if port < 1024 {
        return Err(report!(ConfigError::Reserved(port)));
    }
    Ok(port)
}

// untyped: any error, with additional context for the log
fn start_server(path: &str) -> Result<(), Report> {
    let port = read_port(path)
        .context("server could not start")
        .attach(format!("config file: {path}"))?;
    println!("listening on port {port}");
    Ok(())
}

fn main() {
    // the typed error can be inspected
    if let Err(report) = read_port("port.txt") {
        match report.current_context() {
            ConfigError::Io => println!("no config file, using default port 8080"),
            other => println!("{other}"),
        }
    }

    // the untyped report prints the whole chain
    if let Err(report) = start_server("port.txt") {
        eprintln!("{report}");
    }
}
