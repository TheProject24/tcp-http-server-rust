use crate::server::Server;
mod server;

use std::io::Write;
use tcp_http_server_rust::response::StatusCode;

fn handler(
    w: &mut dyn Write,
    req: &tcp_http_server_rust::request::Request,
) -> Result<(), server::HandlerError> {
    let target = req
        .request_line
        .as_ref()
        .map(|line| line.request_target.as_str())
        .unwrap_or("/");

    match target {
        "/test" => Err(server::HandlerError {
            status_code: StatusCode::BadRequest,
            message: "test bad case\n".to_string(),
        }),
        "/err" => Err(server::HandlerError {
            status_code: StatusCode::InternalServerError,
            message: "err\n".to_string(),
        }),
        _ => {
            w.write_all(b"good\n")
                .map_err(|_| server::HandlerError {
                    status_code: StatusCode::InternalServerError,
                    message: "Woopsie, my bad\n".to_string(),
                })?;
            Ok(())
        }
    }
}

fn main() {
    let server = Server::serve(handler);
    let listener = server.listen();
    println!("IN SERVER");
    listener.join().unwrap();
    println!("close server");
}
