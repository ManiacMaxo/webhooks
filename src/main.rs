use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;

mod utils;

#[tokio::main]
async fn main() {
    let options = utils::cli::args();
    println!("options: {:?}", options);
    let addr = SocketAddr::from(([127, 0, 0, 1], options.port));

    if !Path::new(&options.config_file).exists() {
        println!("config file \"{}\" does not exist", options.config_file);
        return;
    }

    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(utils::server::handler))
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(utils::server::shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
