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
        let crlf_found = data.windows(2).position(|c| c == b"\r\n");
        println!("LINE: {}", str::from_utf8(&data[..]).unwrap());
        if let Some(crlf_pos) = crlf_found {
            // we have full line
            let line = str::from_utf8(&data[..crlf_pos]).unwrap();
            println!("LINE: {}", line);
            if line.is_empty() {
                return Ok((2, true));
            }

            let arr: Vec<&str> = line.split(": ").collect();
            println!("{:?}", arr);
            if arr.len() != 2 {
                return Err("not enough arguments in req body".to_string());
            }
            let key = arr[0];
            let value = arr[1].trim();
            self.headers.insert(key.to_string(), value.to_string());
            Ok((line.len() + 2, false))
        } else {
            Ok((0, false))
        }
    }
}
