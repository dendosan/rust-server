#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let addr = String::from("127.0.0.1");
    let port = String::from("8080");

    let server = Server::new(addr, port);
    server.run(WebsiteHandler);
}
