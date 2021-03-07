mod cache;
mod render;

use clap::Clap;
use log::{error, info};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode};

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(short, long, about = "Path to the world dir")]
    world_dir: String,

    #[clap(short, long, about = "Path to the output dir")]
    output_dir: String,

    #[clap(short, long, about = "Number of render threads", default_value="1")]
    threads: String,
}

fn main() {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    let opts: Opts = Opts::parse();

    info!("Starting flatmap with {} render threads", opts.threads)
}
