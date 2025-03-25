use clap::Parser;
use simulation::run_simulation;

mod components;
mod integration_tests;
mod simulation;
mod stats;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the data file
    #[arg(short, long, default_value = "data.csv")]
    path: String,

    /// Delay (in millis) between radar scans
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,

    /// Channel size between components
    #[arg(short, long, default_value_t = 255)]
    channel_size: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // Set up log levels
    env_logger::init();
    run_simulation(args.delay, &args.path, args.channel_size)
        .await
        .expect("io error");
}
