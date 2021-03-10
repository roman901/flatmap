mod anvil;
mod cache;
mod mcmod;
mod render;

use crate::anvil::{Region, RegionError};
use crate::mcmod::MCMod;
use clap::Clap;
use log::{error, info};
use regex::Regex;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::fs;
use std::fs::FileType;
use std::io::Error;
use std::path::Path;
use std::process::exit;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(short, long, about = "Path to the world dir")]
    world_dir: String,

    #[clap(short, long, about = "Path to the output dir")]
    output_dir: String,

    #[clap(short, long, about = "Number of render threads", default_value = "1")]
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

        // let mcmod = MCMod::from_file(entry.path().as_ref())?;
    }

    let data_path = Path::new(&opts.output_dir);
    if !data_path.exists() {
        fs::create_dir_all(data_path)?;
    }

    let world_path = Path::new(&opts.world_dir);
    if !world_path.exists() {
        error!("World folder {} doesn't exists!", opts.world_dir);
        exit(-1)
    }

    info!("Render world {}", opts.world_dir.as_str());
    let regions_dir = format!("{}/region", opts.world_dir);

    // let pool = ThreadPool::new(1);
    for entry in fs::read_dir(regions_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }

        // pool.execute(move || {
        let position_re = Regex::new(r"r.(-?\d+).(-?\d+).mca").unwrap();
        let region_pathbuf = entry.path();
        let region_path = region_pathbuf.as_path();
        let region_file_name = region_pathbuf.to_string_lossy().into_owned();

        let coords = position_re.captures(region_file_name.as_str()).unwrap();

        let region_x = coords
            .get(1)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .unwrap();
        let region_z = coords
            .get(2)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .unwrap();

        info!("Processing {} {}...", region_x, region_z);
        let region = match Region::from_file(region_path) {
            Ok(region) => region,
            Err(err) => {
                match err {
                    RegionError::EmptyRegionError() => {}
                    _ => error!(
                        "Got error while processing {} {}: {}",
                        region_x, region_z, err
                    ),
                }

                continue;
            }
        };

        // });
    }

    Ok(())
}
