mod cli;

fn main() {
    let options = cli::args();

    println!("{} {} {}", options.config_file, options.port, options.watch);
}
