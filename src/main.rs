use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let addr = String::from("127.0.0.1");
    let port = String::from("8080");

    let server = Server::new(addr, port);
    server.run();
}
