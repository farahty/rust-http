use http::Server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8081".to_string());
    server.run()
}
