use crate::server::Server;
mod server;

fn main() {
    let server = Server::serve();
    let listener = server.listen();
    println!("IN SERVER");
    listener.join().unwrap();
    println!("close server");
}
