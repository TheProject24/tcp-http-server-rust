use std::io::{BufRead, BufReader, Read};

use crate::body::Body;
use crate::headers::Headers;

#[derive(PartialEq)]
enum ParseState {
    Initialized,
    Done,
}

#[derive(PartialEq)]
enum ParsingPart {
    ReqLine,
    ReqHeaders,
    ReqBody,
}

pub struct Request {
    pub request_line: Option<RequestLine>,
    pub request_headers: Headers,
    pub parsing_part: ParsingPart,
    parse_state: ParseState,
}

pub struct RequestLine {
    pub http_version: String,
    pub request_target: String,
    pub method: String,
}

impl Request {
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Request, String> {
        let mut small_buf = [0u8; 8];
        let mut buf: Vec<u8> = Vec::new();
        let mut request = Request {
            request_line: None,
            request_headers: Headers::new(),
            parsing_part: ParsingPart::ReqLine,
            parse_state: ParseState::Initialized,
        };

        while request.parsing_part != ParsingPart::ReqBody {
            let parse_res = request.parse(&buf)?;

            if parse_res > 0 {
                buf.drain(..parse_res);
            }

            if request.parsing_part == ParsingPart::ReqBody {
                break;
            }

            if parse_res == 0 {
                let n = match reader.read(&mut small_buf) {
                    Ok(0) => return Err("no more data in reader".to_string()),
                    Ok(n) => n,
                    Err(_) => return Err("Error reading from stream".to_string()),
                };
                buf.extend_from_slice(&small_buf[..n]);
            }
        }
        Ok(request)
    }

    fn parse(&mut self, data: &[u8]) -> Result<usize, String> {
        if self.parse_state == ParseState::Initialized && self.parsing_part == ParsingPart::ReqLine
        {
            let (size, req_line) = parse_request_line(data)?;
            if size == 0 {
                return Ok(0);
            } else {
                self.request_line = req_line;
                self.parse_state = ParseState::Done;
                self.parsing_part = ParsingPart::ReqHeaders;
                return Ok(size);
            }
        } else if self.parsing_part == ParsingPart::ReqHeaders {
            let (size, done) = self.request_headers.parse(data)?;
            if size == 0 {
                return Ok(0);
            } else if !done {
                return Ok(size);
            } else {
                self.parsing_part = ParsingPart::ReqBody;
                return Ok(size);
            }
        } else {
            println!("parsing body");
            self.request_headers.get("Content-length".to_string());
        }
        return Ok(0);
    }
}

fn parse_request_line(data: &[u8]) -> Result<(usize, Option<RequestLine>), String> {
    let crlf_found = data.windows(2).position(|c| c == b"\r\n");
    if let Some(crlf_pos) = crlf_found {
        // we have full line
        let line = str::from_utf8(&data[..crlf_pos]).unwrap();

        let arr: Vec<&str> = line.split(' ').collect();
        if arr.len() != 3 {
            return Err("not enough arguments in req line".to_string());
        }
        let method = arr[0];
        if !method.chars().all(|c| c.is_ascii_uppercase()) {
            return Err("method needs to be uppercase chars".to_string());
        }
        let request_target = arr[1];
        let http_version = arr[2];
        let real_version: Vec<&str> = http_version.split('/').collect();
        let version = real_version[1].trim_end();
        if version != "1.1" {
            return Err("invalid http version".to_string());
        }
        let res = RequestLine {
            http_version: version.to_string(),
            request_target: request_target.to_string(),
            method: method.to_string(),
        };
        Ok((line.len() + 2, Some(res)))
    } else {
        Ok((0, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ChunkReader {
        data: Vec<u8>,
        num_bytes_per_read: usize,
        pos: usize,
    }

    impl ChunkReader {
        fn new(data: &str, num_bytes_per_read: usize) -> Self {
            ChunkReader {
                data: data.as_bytes().to_vec(),
                num_bytes_per_read,
                pos: 0,
            }
        }
    }

    impl Read for ChunkReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.pos >= self.data.len() {
                return Ok(0);
            }

            // Only read num_bytes_per_read at a time (simulates chunked arrival)
            let end = std::cmp::min(self.pos + self.num_bytes_per_read, self.data.len());
            let bytes_to_read = std::cmp::min(end - self.pos, buf.len());

            buf[..bytes_to_read].copy_from_slice(&self.data[self.pos..self.pos + bytes_to_read]);
            self.pos += bytes_to_read;

            Ok(bytes_to_read)
        }
    }

    #[test]
    fn test_good_get_request_line() {
        let input = "GET / HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";
        let reader = ChunkReader::new(input, 3);
        let request = Request::from_reader(reader).unwrap();

        let req_line = request.request_line.unwrap();

        assert_eq!(req_line.method, "GET");
        assert_eq!(req_line.http_version, "1.1");
        assert_eq!(req_line.request_target, "/");
    }

    #[test]
    fn test_good_get_request_line_with_path() {
        let input = "GET /joe HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";
        let reader = ChunkReader::new(input, 3);
        let request = Request::from_reader(reader).unwrap();

        let req_line = request.request_line.unwrap();

        assert_eq!(req_line.method, "GET");
        assert_eq!(req_line.http_version, "1.1");
        assert_eq!(req_line.request_target, "/joe");
    }

    #[test]
    fn test_bad_get_request_line() {
        let input = "GET HTTP/1.1\r\nHost: localhost:3000\r\n\r\n";
        let request = Request::from_reader(input.as_bytes());

        assert!(request.is_err());
    }

    #[test]
    fn test_parse_headers_standard_headers() {
        let input = "GET / HTTP/1.1\r\nHost: localhost:42069\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n";
        let reader = ChunkReader::new(input, 3);
        let request = Request::from_reader(reader).unwrap();

        assert_eq!(
            request.request_headers.headers.get("host").unwrap(),
            "localhost:42069"
        );
        assert_eq!(
            request.request_headers.headers.get("user-agent").unwrap(),
            "curl/7.81.0"
        );
        assert_eq!(
            request.request_headers.headers.get("accept").unwrap(),
            "*/*"
        );
    }

    #[test]
    fn test_parse_headers_malformed_header() {
        let malformed = "GET / HTTP/1.1\r\nHost localhost:42069\r\n\r\n";
        let reader = ChunkReader::new(malformed, 3);
        let request = Request::from_reader(reader);

        assert!(request.is_err());
    }
}
