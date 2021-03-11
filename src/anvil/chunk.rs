use crate::anvil::section::Section;
use crate::anvil::AnvilError;
use byteorder::{BigEndian, ReadBytesExt};
use nbt::de::{from_gzip_reader, from_zlib_reader};
use std::io::Cursor;

use serde::Deserialize;

#[derive(Debug)]
pub struct Chunk {
    pub x: u8,
    pub z: u8,
    // pub data_version: u32,
    pub sections: Vec<Section>,
}
impl Chunk {
    pub fn read(pos: usize, buf: &mut Vec<u8>) -> Result<Chunk, AnvilError> {
        let mut sections = Vec::new();
        let mut cur = Cursor::new(&buf);
        let _length = cur.read_u32::<BigEndian>()?;
        let format = cur.read_u8()?;

        let blob: ChunkNBT = match format {
            1 => from_gzip_reader(&mut cur)?,
            2 => from_zlib_reader(&mut cur)?,
            _ => return Err(AnvilError::UnsupportedChunkCompression()),
        };

        let x = (pos / 32) as u8;
        let z = (pos % 32) as u8;

        println!("{:#?}", blob);
        //println!("{} {}", blob.len_bytes(), _length);
        // let data_version = blob.get::<Value::Int>("DataVersion")?.into();
        Ok(Chunk { x, z, sections })
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ChunkNBT {
    DataVersion: u32,
    Level: ChunkLevelNBT,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ChunkLevelNBT {
    LastUpdate: u32,
}
