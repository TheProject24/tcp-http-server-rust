use crate::server::Server;
mod server;

use std::io::Write;
use tcp_http_server_rust::response::StatusCode;

/// The core request handler for our HTTP server.
/// 
/// This function takes a writer (to write the HTTP response) and the parsed `Request` object.
/// It routes the request based on the path (request target). It simulates different 
/// responses like `400 Bad Request` or `500 Internal Server Error` based on specific paths,
/// or otherwise safely defaults to a simple "good" response string.
fn handler(
    w: &mut dyn Write,
    req: &tcp_http_server_rust::request::Request,
) -> Result<(), server::HandlerError> {
    // Extract the target path. Defaults to root "/" if parsing failed or was absent.
    let target = req
        .request_line
        .as_ref()
        .map(|line| line.request_target.as_str())
        .unwrap_or("/");

    // Match the requested path to specific application routes
    match target {
        // Simulates a bad request error case
        "/test" => Err(server::HandlerError {
            status_code: StatusCode::BadRequest,
            message: "test bad case\n".to_string(),
        }),
        // Simulates an internal server error case
        "/err" => Err(server::HandlerError {
            status_code: StatusCode::InternalServerError,
            message: "err\n".to_string(),
        }),
        // Catch-all block for standard successful routing (returns HTTP 200 OK by default)
        _ => {
            // Write our raw response data back to the body
            w.write_all(b"good\n")
                // Propagate a server error if writing to the buffer fails
                .map_err(|_| server::HandlerError {
                    status_code: StatusCode::InternalServerError,
                    message: "Woopsie, my bad\n".to_string(),
                })?;
            Ok(())
        }
    }
}

/// The executable entry point for the HTTP server application.
/// 
/// This runs the listening loop on the main server thread, delegates incoming TCP 
/// streams to the predefined routing `handler`, and gracefully waits for the listener
/// to complete.
fn main() {
    // Initialise the server with our provided route handler logic
    let server = Server::serve(handler);
    // Start listening for connections and spawn background processing logic
    let listener = server.listen();
    println!("IN SERVER");
    
    // Block the current main thread until the listener thread completes
    listener.join().unwrap();
    println!("close server");
}
