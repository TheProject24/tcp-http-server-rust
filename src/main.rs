use std::io::{self, Read};
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver};
use std::thread;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069")?;
    for stream in listener.incoming() {
        let rx = get_lines_channel(stream?);
        println!("Connection established");
        for line in rx {
            println!("{}", line);
        }
        println!("Connection closed");
    }
    Ok(())
}

fn get_lines_channel<R>(mut reader: R) -> Receiver<String>
where
    R: Read + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let mut buf = [0u8; 8];
        let mut pending = String::from("");
        loop {
            let n = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };

            let chunk = String::from_utf8_lossy(&buf[..n]);
            let mut start = 0usize;
            for (i, ch) in chunk.char_indices() {
                if ch == '\n' {
                    pending.push_str(&chunk[start..i]);
                    start = i + 1;
                    let _ = tx.send(pending.clone());
                    pending.clear();
                }
            }
            pending.push_str(&chunk[start..]);
        }

        if !pending.is_empty() {
            let _ = tx.send(pending);
        }
    });

    rx
}
