mod cache;
mod render;
mod mcmod;
mod map;

use clap::Clap;
use log::{error, info};
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode};
use std::fs;
use std::io::Error;
use std::path::Path;
use std::fs::FileType;
use crate::mcmod::MCMod;

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

fn main() -> Result<(), Error> {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout).unwrap();
    let opts: Opts = Opts::parse();

    info!("Starting flatmap with {} render threads", opts.threads);

    fs::create_dir_all("mods")?;

    for entry in fs::read_dir(Path::new("mods"))? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }
        info!("Found mod {:?}", entry.file_name());

        let mcmod = MCMod::from_file(entry.path().as_ref())?;
    }

    Ok(())
}
