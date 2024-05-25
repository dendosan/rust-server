use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

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
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {:?}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish connection: {} ", e),
            }
        }
    }
}
