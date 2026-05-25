use std::io::{self, BufRead, Write};
use std::net::UdpSocket;

/// Interactive CLI executable mimicking basic raw UDP client socket communication.
/// It takes standard user line input and pumps it across bound UDP configurations 
/// to test backend parsers independently of typical HTTP TCP flows.
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind to socket");

    socket
        .connect("127.0.0.1:42069")
        .expect("failed to connect");

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        reader.read_line(&mut line).expect("failed to read line");

        socket.send(line.as_bytes()).expect("failed to send on udp");
    }
}
