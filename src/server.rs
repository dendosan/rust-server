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
        let full_addr = [&self.addr, ":", &self.port].concat();
        println!("Listening on {}", full_addr);

        let listener = TcpListener::bind(full_addr).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("OK");
                },
                Err(e) => println!("Failed to establish connection: {} ", e),
            }
        }
    }
}
