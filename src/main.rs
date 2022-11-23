pub mod config;

fn main() {
    let config = crate::config::Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}. Please provide at one query string and a file containing the data");
        std::process::exit(1);
    });
    println!("Looking for search query: {}", config.query);

    if let Err(e) = crate::config::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
