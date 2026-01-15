use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("messages.txt")?;
    let mut buf = [0u8; 8];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        println!("read: {}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}
