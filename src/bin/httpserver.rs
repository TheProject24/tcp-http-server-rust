mod server;

fn main() {
    server::serve(port);
    server.close();
}
