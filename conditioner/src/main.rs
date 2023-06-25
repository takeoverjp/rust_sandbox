use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    println!("Hello, world! {:?}", args);
    tracing::info!("Hello, world! {:?}", args);
    log::info!("Hello, world! {:?}", args);
}
