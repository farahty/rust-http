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

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("{:?}", addr);
                    println!("{:?}", stream);
                }
                Err(_) => todo!(),
            }
        }
    }
}

pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    PUT,
    POST,
    DELETE,
    PATCH,
}

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
