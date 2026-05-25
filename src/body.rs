use std::collections::HashMap;

use crate::headers::Headers;

/// Context managing the underlying payload or content of an HTTP response or request.
pub struct Body {
    /// Raw bytes containing the parsed or to-be-sent body.
    pub body: Vec<u8>,
}

impl Body {
    /// Initialises a new `Body` struct context with an empty vector.
    pub fn new() -> Body {
        Body { body: Vec::new() }
    }

    /// Placeholder function representing future appending capabilities.
    pub fn append(self) -> Result<(usize, bool), String> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_body() {
        // let mut body = Body::new();
        // let data = b"Content-Length: 13\r\n
        //                                 \r\n
        //                                 hello world!\n";
        // let (n, done) = body.parse(data).unwrap();
        //
        // assert_eq!(headers.headers.get("host").unwrap(), "localhost:42069");
        // assert_eq!(n, 23);
        // assert!(!done);
    }
}
