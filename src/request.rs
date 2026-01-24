use std::io::{BufRead, BufReader, Read};

pub struct Request {
    request_line: RequestLine,
}

pub struct RequestLine {
    http_version: String,
    request_target: String,
    method: String,
}

impl Request {
    pub fn from_reader<R: Read>(reader: R) -> Result<Request, String> {
        let mut req_reader = BufReader::new(reader);
        let mut line = String::new();
        req_reader.read_line(&mut line);
        let arr: Vec<&str> = line.split(' ').collect();
        if arr.len() != 3 {
            return Err("err".to_string());
        }
        let method = arr[0];
        let request_target = arr[1];
        let http_version = arr[2];
        let real_version: Vec<&str> = http_version.split('/').collect();
        let version = real_version[1].trim_end();
        let res = RequestLine {
            http_version: version.to_string(),
            request_target: request_target.to_string(),
            method: method.to_string(),
        };
        Ok(Request { request_line: res })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_get_request_line() {
        let input = "GET / HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";
        let request = Request::from_reader(input.as_bytes()).unwrap();

        assert_eq!(request.request_line.method, "GET");
        assert_eq!(request.request_line.http_version, "1.1");
        assert_eq!(request.request_line.request_target, "/");
    }

    #[test]
    fn test_good_get_request_line_with_path() {
        let input = "GET /joe HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";
        let request = Request::from_reader(input.as_bytes()).unwrap();

        assert_eq!(request.request_line.method, "GET");
        assert_eq!(request.request_line.http_version, "1.1");
        assert_eq!(request.request_line.request_target, "/joe");
    }

    #[test]
    fn test_bad_get_request_line() {
        let input = "GET HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";
        let request = Request::from_reader(input.as_bytes());

        assert!(request.is_err());
    }
}
