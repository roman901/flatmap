use std::path::Path;
use std::fs::{File};
use thiserror::Error;
use nbt::Blob;
use log::{info, error, debug};
use std::io::{SeekFrom, Seek, Read};


#[derive(Error, Debug)]
pub enum RegionError {
    #[error(transparent)]
    ReadError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] nbt::Error),
    #[error("Found empty region file")]
    EmptyRegionError(),

}

pub struct Region {

}
impl Region {
    pub fn from_file(path: &Path) -> Result<Region, RegionError> {
        let mut file = File::open(path)?;
        if file.metadata()?.len() == 0 {
            return Err(RegionError::EmptyRegionError())
        }
        file.seek(SeekFrom::Start(0))?;

        for i in 0..2 {
            debug!("Processing chunk {}", i);
            let mut buf = [0; 4];
            file.seek(SeekFrom::Current(4))?;
            file.read_exact(&mut buf)?;
            println!("{:?}", buf);
        }
        /*let blob = Blob::from_zlib_reader(&mut file)?;

        println!("{:?}", blob);*/
        Ok(Region{

        })
    }
}

struct Chunk {

}

struct Section {

}

struct Block {

}