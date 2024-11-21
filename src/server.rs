use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listner = TcpListener::bind(self.address).unwrap();

        // infinte loop
        loop {
            match listner.accept(){
                Ok((stream, _)) => {
                   println!("New connection from {}", stream.peer_addr().unwrap());
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                    continue;
                }
            }
        }
    }
}