use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("server is listening on address {}", &self.addr);
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let data = String::from_utf8_lossy(&buffer);
                            println!("data received: \n{}", &data);

                            match Request::try_from(&buffer[..]) {
                                Ok(_) => todo!(),
                                Err(_) => todo!(),
                            }
                        }
                        Err(e) => {
                            println!("failed to read from a connection {}", e);
                        }
                    }

                    println!("{:?}", addr);
                    println!("{:?}", stream);
                }
                Err(_) => println!("failed to establish connection at {}", &self.addr),
            }
        }
    }
}
