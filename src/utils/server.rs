use crate::utils::config::ConfigMap;
use httparse::{Header, Request, EMPTY_HEADER};
use std::collections::HashMap;
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
        stream.read(&mut buf).expect("failed to read request");

        let mut headers = [EMPTY_HEADER; 16];

        let mut req = Request::new(&mut headers);
        let res = req.parse(&buf).unwrap();

        if res.is_partial() {
            println!("request is not complete");
            match req.path {
                Some(ref path) => {
                    // check path

                    let conf = self.config.get(&path.to_string());
                    println!("path: {:?} {:?}", path, conf);
                }
                None => {
                    // must read more and parse again
                }
            }
        } else {
            println!("request is complete: {:?}", self.parse_headers(req.headers));
        }

        // close the connection
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn parse_headers(&self, headers: &[Header]) -> HashMap<String, String> {
        let mut headers_map = HashMap::<String, String>::new();

        for header in headers {
            headers_map.insert(
                header.name.to_string(),
                String::from_utf8(header.value.to_vec()).unwrap(),
            );
        }

        return headers_map;
    }
}
