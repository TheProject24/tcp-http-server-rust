use std::collections::HashMap;

pub struct Headers {
    pub headers: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: HashMap::new(),
        }
    }
    pub fn parse(&mut self, data: &[u8]) -> Result<(usize, bool), String> {
        const ALLOWED_CHARS: &[u8] = b"!#$%&'*+-.^_`|~";
        let crlf_found = data.windows(2).position(|c| c == b"\r\n");
        if let Some(crlf_pos) = crlf_found {
            // we have full line
            let line = str::from_utf8(&data[..crlf_pos]).unwrap().trim();
            if line.is_empty() {
                return Ok((2, true));
            }

            let arr: Vec<&str> = line.split(": ").collect();
            if arr.len() != 2 {
                return Err("not enough arguments in req body".to_string());
            }
            let key = arr[0].to_lowercase();
            if key.chars().last() == Some(' ') {
                return Err("invalid spacing in req body".to_string());
            }
            if !key
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || ALLOWED_CHARS.contains(&b))
            {
                return Err("header key needs to be valid ascii".to_string());
            }
            let value = arr[1].trim();
            if self.headers.contains_key(&key) {
                let curr_value = self.headers.get(&key).unwrap();
                let mut new_value = curr_value.to_string();
                new_value.push_str(", ");
                new_value.push_str(value);
                self.headers.insert(key, new_value);
            } else {
                self.headers.insert(key.to_string(), value.to_string());
            }
            Ok((crlf_pos + 2, false))
        } else {
            Ok((0, false))
        }
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.headers.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_single_header() {
        let mut headers = Headers::new();
        let data = b"Host: localhost:42069\r\n\r\n";
        let (n, done) = headers.parse(data).unwrap();

        assert_eq!(headers.headers.get("host").unwrap(), "localhost:42069");
        assert_eq!(n, 23);
        assert!(!done);
    }

    #[test]
    fn test_valid_single_header_with_extra_whitespace() {
        let mut headers = Headers::new();
        let data = b"   Host: localhost:42069   \r\n\r\n";
        let (n, done) = headers.parse(data).unwrap();

        assert_eq!(headers.headers.get("host").unwrap(), "localhost:42069");
        assert!(!done);
    }

    #[test]
    fn test_valid_duplicate_header() {
        let mut headers = Headers::new();
        let data = b"   Host: localhost:42069   \r\n\r\n";
        let data2 = b"   Host: localhost:1 \r\n\r\n";
        let (n, done) = headers.parse(data).unwrap();
        let (n, done) = headers.parse(data2).unwrap();

        assert_eq!(
            headers.headers.get("host").unwrap(),
            "localhost:42069, localhost:1"
        );
        assert!(!done);
    }

    #[test]
    fn test_valid_done() {
        let mut headers = Headers::new();
        let data = b"\r\n";
        let (n, done) = headers.parse(data).unwrap();

        assert_eq!(n, 2);
        assert!(done);
        assert!(headers.headers.is_empty());
    }

    #[test]
    fn test_invalid_spacing_header() {
        let mut headers = Headers::new();
        let data = b"       Host : localhost:42069       \r\n\r\n";
        let result = headers.parse(data);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_chars_header() {
        let mut headers = Headers::new();
        let data = b"H\xC2\xA9st: localhost:42069\r\n\r\n";
        let result = headers.parse(data);

        assert!(result.is_err());
    }
}
