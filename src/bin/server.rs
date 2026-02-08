use std::io::Write;
use std::net::TcpListener;
use std::thread;

use tcp_http_server_rust::request::Request;

pub struct Server {
    listener: TcpListener,
    closed: bool,
}

impl Server {
    pub fn serve() -> Server {
        Server {
            listener: TcpListener::bind("127.0.0.1:42069").unwrap(),
            closed: false,
        }
    }

    pub fn listen(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for incoming in self.listener.incoming() {
                if self.closed {
                    break;
                }

                let mut stream = match incoming {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("test");
                        continue;
                    }
                };

                match Request::from_reader(&mut stream) {
                    Ok(_req) => {
                        println!("parsing req");
                        if let Err(e) = stream.write_all(
                            b"HTTP/1.1 200 OK\r\n
                                Content-Type: text/plain\r\n
                                Content-Length: 13\r\n
                                \r\n
                                hello world!",
                        ) {
                            eprintln!("write err {}", e);
                            continue;
                        }
                        if let Err(e) = stream.flush() {
                            eprintln!("flush err: {}", e);
                            continue;
                        }
                    }
                    Err(e) => {
                        eprintln!("err: {}", e);
                    }
                }
            }
        })
    }

    pub fn close(self) {}
}

fn main() {}
