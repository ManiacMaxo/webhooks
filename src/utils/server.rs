use crate::utils::config::Config;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Server {
    pub config: Vec<Config>,
    pub listener: TcpListener,
}

impl Server {
    pub fn new(config: Vec<Config>, port: u16) -> Server {
        let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).unwrap();

        return Server {
            config: config,
            listener: listener,
        };
    }

    pub fn request_handler(&self, mut stream: TcpStream) {
        println!("Connection established! {:?}", stream.local_addr());

        // read the request
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        // close the connection
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
