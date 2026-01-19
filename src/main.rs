use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("messages.txt")?;
    let mut buf = [0u8; 8];
    let mut curr_line = String::from("");

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let str = String::from_utf8_lossy(&buf[..n]);
        if str.contains("\n") {
            let split_str: Vec<&str> = str.split("\n").collect();
            let first = split_str[0];
            curr_line.push_str(first);
            println!("read: {}", curr_line);
            curr_line = String::from("");
            let second = split_str[1];
            curr_line.push_str(second);
        } else {
            curr_line.push_str(&str);
        }
    }
    // println!("read: {}", curr_line);

    Ok(())
}
