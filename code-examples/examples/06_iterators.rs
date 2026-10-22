fn main() {
    let lines = [
        "INFO server started",
        "ERROR disk full",
        "INFO request handled",
        "ERROR connection lost",
    ];
    let level = "ERROR"; // captured by the closure

    let alerts: Vec<String> = lines
        .iter()
        .filter(|line| line.starts_with(level))
        .map(|line| line.to_lowercase())
        .collect(); // ["error disk full", "error connection lost"]

    let info_count = lines.iter().filter(|line| line.starts_with("INFO")).count();
    println!("{alerts:?} {info_count}"); // 2
}
