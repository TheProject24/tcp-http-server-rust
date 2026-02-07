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
            for stream in self.listener.incoming() {
                if self.closed {
                    break;
                }
                match Request::from_reader(stream.expect("test")) {
                    Ok(req) => {
                        println!("parsing req");
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
