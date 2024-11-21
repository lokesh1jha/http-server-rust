fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;

    let server = server::Server::new("127.0.0.1:8000".to_string());
    server.run();
}

mod server {
    pub struct Server {
        address: String,
    }

    impl Server {
        pub fn new(address: String) -> Self {
            Server { address }
        }

        pub fn run(self) {
            println!("Listening on {}", self.address);
        }
    }
}

mod http {

    mod request {
        struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }
    // in enum things are takes as index of 0
    // like 0- GET, 1- POST
    mod method {
        pub enum Method {
            GET,
            POST,
            PUT,
            PATCH,
            DELETE,
            OPTIONS,
            HEAD,
        }
    }
}
