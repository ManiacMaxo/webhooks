use std::net::SocketAddr;

mod utils;

fn main() {
    let options = utils::cli::args();
    println!("options: {:?}", options);

    let config = utils::config::load_config(options.config_file).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], options.port));

    let server = utils::server::Server::new(config, &addr);

    for stream in server.listener.incoming() {
        server.request_handler(stream.unwrap())
    }
}
