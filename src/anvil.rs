use byteorder::{BigEndian, ReadBytesExt};
use log::{debug, error, info};
use nbt::Blob;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegionError {
    #[error(transparent)]
    ReadError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] nbt::Error),
    #[error("Found empty region file")]
    EmptyRegionError(),
    #[error("Found unsupported chunk compression algorithm")]
    UnsupportedChunkCompression(),
}

pub struct Region {
    pub chunks: Vec<Chunk>,
}
impl Region {
    pub fn from_file(path: &Path) -> Result<Region, RegionError> {
        let mut file = File::open(path)?;
        if file.metadata()?.len() == 0 {
            return Err(RegionError::EmptyRegionError());
        }
        file.seek(SeekFrom::Start(0))?;
        let mut chunk_offsets = [0u8; 4096];
        file.read_exact(&mut chunk_offsets)?;

        let mut chunks = Vec::new();

        for i in 0..1 {
            debug!("Processing chunk {}", i);
            let mut chunk_location = &chunk_offsets[i * 4..i * 4 + 4];
            let chunk_location = chunk_location.read_u32::<BigEndian>()?;

            let offset = ((chunk_location >> 8) & 0xFFFFFF) * 4096;
            let size = (chunk_location & 0xFF) * 4096;
            println!("{:?} {:?}", offset, size);

            if size == 0 {
                continue;
            }

            let mut chunk_buf = vec![0u8; size as usize];
            file.seek(SeekFrom::Start(offset as u64))?;
            file.read_exact(&mut chunk_buf)?;

            let chunk = Chunk::read(&mut chunk_buf)?;
            println!("{:?}", chunk);
            chunks.push(chunk);

        }

        Ok(Region { chunks })
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub sections: Vec<Section>,
}
impl Chunk {
    pub fn read(buf: &mut Vec<u8>) -> Result<Chunk, RegionError> {
        let mut sections = Vec::new();
        let mut cur = Cursor::new(&buf);
        let _length = cur.read_u32::<BigEndian>()?;
        let format = cur.read_u8()?;

        let blob = match format {
            1 => Blob::from_gzip_reader(&mut cur)?,
            2 => Blob::from_zlib_reader(&mut cur)?,
            _ => return Err(RegionError::UnsupportedChunkCompression()),
        };

        println!("{:?}", blob);
        Ok(Chunk { sections })
    }
}

#[derive(Debug)]
pub struct Section {}

#[derive(Debug)]
pub struct Block {}
