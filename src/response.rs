use crate::headers::Headers;
use std::io::{self, Write};

/// Strongly typed enum for standard HTTP status codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    InternalServerError = 500,
}

/// Generates and writes the standard HTTP/1.1 response status line.
/// 
/// Constructs a string formatted as `"HTTP/1.1 {code} {Phrase}\r\n"`, writes it to
/// the stream writer `w`, and propagates any underlying IO errors.
pub fn write_status_line<W: Write>(w: &mut W, status: StatusCode) -> io::Result<()> {
    let mut http_string = String::from("HTTP/1.1 ");
    let code = status as u16;
    http_string.push_str(&code.to_string());

    match status {
        StatusCode::Ok => http_string.push_str(" OK"),
        StatusCode::BadRequest => http_string.push_str(" Bad Request"),
        StatusCode::InternalServerError => http_string.push_str(" Internal Server Error"),
    }

    http_string.push_str("\r\n");
    w.write_all(http_string.as_bytes())?;
    Ok(())
}

/// Returns a pre-populated set of `Headers` indicating the standard response
/// parameters like `Content-Length`, `Connection: close`, and `Content-Type`.
pub fn get_default_headers(content_len: usize) -> Headers {
    let mut default_headers = Headers::new();
    default_headers.headers.insert("content-length".to_string(), content_len.to_string());
    default_headers.headers.insert("connection".to_string(), "close".to_string());
    default_headers.headers.insert("content-type".to_string(), "text/plain".to_string());
    default_headers
}

/// Iterates over a parsed internal `Headers` structure and writes these attributes
/// directly to the stream. Automatically appends trailing boundaries like `\r\n`.
pub fn write_headers<W: Write>(w: &mut W, headers: &Headers) -> io::Result<()> {
    for (key, value) in &headers.headers {
        let mut line = key.to_string();
        line.push_str(": ");
        line.push_str(&value.to_string());
        line.push_str("\r\n");
        w.write_all(line.as_bytes())?;
    }
    w.write_all("\r\n".as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_200() {
        let mut out = Vec::new();
        write_status_line(&mut out, StatusCode::Ok).unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "HTTP/1.1 200 OK\r\n");
    }

    #[test]
    fn test_400() {
        let mut out = Vec::new();
        write_status_line(&mut out, StatusCode::BadRequest).unwrap();

        assert_eq!(
            String::from_utf8(out).unwrap(),
            "HTTP/1.1 400 Bad Request\r\n"
        );
    }

    #[test]
    fn test_500() {
        let mut out = Vec::new();
        write_status_line(&mut out, StatusCode::InternalServerError).unwrap();

        assert_eq!(
            String::from_utf8(out).unwrap(),
            "HTTP/1.1 500 Internal Server Error\r\n"
        );
    }

    #[test]
    fn test_default_headers() {
        let headers = get_default_headers(10 as usize);

        assert_eq!(
            headers.headers.get("content-length").unwrap(),
            "10"
        );

        assert_eq!(
            headers.headers.get("connection").unwrap(),
            "close"
        );

        assert_eq!(
            headers.headers.get("content-type").unwrap(),
            "text/plain"
        );
    }
}
