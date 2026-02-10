use std::io::Write;
use std::net::TcpListener;
use std::thread;

use tcp_http_server_rust::request::Request;
use tcp_http_server_rust::response::{
    get_default_headers, write_headers, write_status_line, StatusCode,
};

pub type Handler = fn(w: &mut dyn Write, req: &Request) -> Result<(), HandlerError>;

pub struct Server {
    listener: TcpListener,
    handler: Handler,
    closed: bool,
}

pub struct HandlerError {
    pub status_code: StatusCode,
    pub message: String,
}

impl Server {
    pub fn serve(handler: Handler) -> Server {
        Server {
            listener: TcpListener::bind("127.0.0.1:42069").unwrap(),
            handler,
            closed: false,
        }
    }

    pub fn listen(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let handler = self.handler;
            for incoming in self.listener.incoming() {
                if self.closed {
                    break;
                }

                let mut stream = match incoming {
                    Ok(s) => s,
                    Err(_) => {
                        eprintln!("test");
                        continue;
                    }
                };

                match Request::from_reader(&mut stream) {
                    Ok(req) => {
                        let mut body_buf: Vec<u8> = Vec::new();
                        let handler_res = handler(&mut body_buf, &req);
                        match handler_res {
                            Ok(()) => {
                                if let Err(e) = write_status_line(&mut stream, StatusCode::Ok) {
                                    eprintln!("write status err: {}", e);
                                    continue;
                                }

                                let headers = get_default_headers(body_buf.len());
                                if let Err(e) = write_headers(&mut stream, &headers) {
                                    eprintln!("write err: {}", e);
                                    continue;
                                }

                                if let Err(e) = stream.write_all(&body_buf) {
                                    eprintln!("write body err: {}", e);
                                    continue;
                                }

                                if let Err(e) = stream.flush() {
                                    eprintln!("flush err: {}", e);
                                    continue;
                                }
                            }
                            Err(err) => {
                                if let Err(e) = write_status_line(&mut stream, err.status_code) {
                                    eprintln!("write status err: {}", e);
                                    continue;
                                }

                                let error_body = err.message.into_bytes();
                                let headers = get_default_headers(error_body.len());
                                if let Err(e) = write_headers(&mut stream, &headers) {
                                    eprintln!("write err: {}", e);
                                    continue;
                                }

                                if let Err(e) = stream.write_all(&error_body) {
                                    eprintln!("write body err: {}", e);
                                    continue;
                                }

                                if let Err(e) = stream.flush() {
                                    eprintln!("flush err: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        if let Err(e) = write_status_line(&mut stream, StatusCode::BadRequest) {
                            eprintln!("write status err: {}", e);
                            continue;
                        }

                        let headers = get_default_headers(0);
                        if let Err(e) = write_headers(&mut stream, &headers) {
                            eprintln!("write err: {}", e);
                            continue;
                        }

                        if let Err(e) = stream.flush() {
                            eprintln!("flush err: {}", e);
                            continue;
                        }
                    }
                }
            }
        })
    }

    pub fn close(self) {}
}

fn main() {}
