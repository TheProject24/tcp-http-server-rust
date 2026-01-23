pub struct Request {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_line_parse() {
        assert_eq!("req_line", "req_line");
    }
}
