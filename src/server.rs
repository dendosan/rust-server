use std::net::TcpListener;

pub struct Server {
    addr: String,
    port: String,
}

impl Server {
    pub fn new(addr: String, port: String) -> Self {
        Self {
            addr,
            port,
        }
    }

    pub fn run(self) {
        println!("Listening on {}:{}", self.addr, self.port);
        let full_addr = [&self.addr, ":", &self.port].concat();

        let listener = TcpListener::bind(full_addr);

    }
}
