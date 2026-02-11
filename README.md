# HTTP Server from scratch in Rust

This is a minimal HTTP/1.1 server built from scratch in Rust. This project implements http request parsing (request line, headers, body) and a server loop that accepts TCP connections and returns a valid HTTP response.

## Features
- HTTP/1.1 request parsing
- Header parsing with case-insensitive lookup
- Body parsing using Content-Length
- Basic TCP server with handler-based responses

## Project Structure
- `src/request.rs` - HTTP request parser
- `src/headers.rs` - Header parsing and lookup
- `src/bin/httpserver.rs` - HTTP server entrypoint
- `src/bin/server.rs` - Server implementation

## Run the server
```
cargo run --bin httpserver
```

Then test it:
```
curl -i http://localhost:8080
```

