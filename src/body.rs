use std::collections::HashMap;

use crate::headers::Headers;

pub struct Body {
    pub body: Vec<u8>,
}

impl Body {
    pub fn new() -> Body {
        Body { body: Vec::new() }
    }

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
