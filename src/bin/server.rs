use std::net::TcpListener;
use std::thread;

struct Server {
    listener: TcpListener,
    closed: bool,
}

impl Server {
    fn serve() -> Server {
        Server {
            listener: TcpListener::bind("127.0.0.1:42069").unwrap(),
            closed: false,
        }
    }

    fn listen(self) {
        let handle = thread::spawn(move || {
            for stream in self.listener.incoming() {
                println!("hello");
            }
        });
    }

    fn close(self) {}
}

fn main() {}
