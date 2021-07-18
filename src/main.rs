mod utils;

fn main() {
    let options = utils::cli::args();

    println!("options: {:?}", options);

    let config = utils::config::load_config(options.config_file).unwrap();
    println!("{:?}", config);

    let server = utils::server::Server::new(config, options.port);

    println!(
        "Listening on http://{}",
        server.listener.local_addr().unwrap()
    );

    for stream in server.listener.incoming() {
        server.request_handler(stream.unwrap())
    }
}
