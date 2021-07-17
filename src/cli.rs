use clap::{App, Arg};

pub struct Options {
    pub config_file: String,
    pub port: u16,
    pub watch: bool,
}

pub fn args() -> Options {
    let port = Arg::from_usage("-p, --port=[INT] 'Port to listen on'");
    let watch = Arg::from_usage("-w, --watch 'Watch for file changes'");

    let config = Arg::with_name("config")
        .index(1)
        .help("the config file to use");

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .args(&[port, watch, config])
        .get_matches();

    return Options {
        config_file: matches
            .value_of("config")
            .unwrap_or("hooks.json")
            .to_string(),
        port: matches
            .value_of("port")
            .unwrap_or("5000")
            .parse::<u16>()
            .unwrap(),
        watch: matches.occurrences_of("watch") > 0,
    };
}
