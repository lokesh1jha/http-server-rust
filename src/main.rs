use server::Server;
use http::Method;
use http::Request;

mod server;
mod http;

fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;

    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
