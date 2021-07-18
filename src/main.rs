use std::net::{SocketAddr, TcpListener};
mod cli;
mod config;

fn main() {
    let options = cli::args();

    println!(
        "options: {} {} {}",
        options.config_file, options.port, options.watch
    );

    let conf = config::load_config(options.config_file).unwrap();
    println!("{:?}", conf);

    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], options.port))).unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established! {:?}", stream);
    }
}
