use crate::utils::config::ConfigMap;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Server {
    pub config: ConfigMap,
    pub listener: TcpListener,
}

impl Server {
    pub fn new(config: ConfigMap, addr: &SocketAddr) -> Server {
        let listener = TcpListener::bind(addr).unwrap();

        return Server {
            config: config,
            listener: listener,
        };
    }

    pub fn request_handler(&self, mut stream: TcpStream) {
        println!("Connection established! {:?}", stream.local_addr());

        // read the request
        let mut buf = [0; 1024];
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        stream.read(&mut buf).unwrap();
        let res = req.parse(&buf).unwrap();

        if res.is_partial() {
            match req.path {
                Some(ref path) => {
                    // check router for path.
                    // /404 doesn't exist? we could stop parsing
                }
                None => {
                    // must read more and parse again
                }
            }
        }

        println!("Request: {:?}", headers);

        // close the connection
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
