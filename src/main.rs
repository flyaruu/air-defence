
use clap::Parser;
use simulation::run_simulation;

mod radar;
mod iff;
mod fire_unit;
mod simulation;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "data.csv")]
    path: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,
}

pub enum IFFMessage {
    Fire,
    IFFShutDown,
}

fn main() {
    let args = Args::parse();
    env_logger::init();
    run_simulation(args.delay, &args.path);
}

