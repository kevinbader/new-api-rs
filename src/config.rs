use clap::Parser;

/// TODO - Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Port
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
