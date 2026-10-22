/// Returns true for ports that need no root privileges.
///
/// ```
/// use assertr::prelude::*;
/// use rust_talk::is_valid_port;
/// assert_that!(is_valid_port(1023)).is_false();
/// assert_that!(is_valid_port(1024)).is_true();
/// ```
pub fn is_valid_port(port: u16) -> bool {
    port >= 1024
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn rejects_reserved_ports() {
        // standard library: assert!(!is_valid_port(80));
        assert_that!(is_valid_port(80)).is_false();
    }
}
