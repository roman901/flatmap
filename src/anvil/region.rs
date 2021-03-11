use crate::anvil::chunk::Chunk;
use crate::anvil::AnvilError;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct Region {
    pub chunks: Vec<Chunk>,
}
impl Region {
    pub fn from_file(path: &Path) -> Result<Region, AnvilError> {
        let mut file = File::open(path)?;
        if file.metadata()?.len() == 0 {
            return Err(AnvilError::EmptyRegionError());
        }
        file.seek(SeekFrom::Start(0))?;
        let mut chunk_offsets = [0u8; 4096];
        file.read_exact(&mut chunk_offsets)?;

        let mut chunks = Vec::new();

        for i in 0..1 {
            let mut chunk_location = &chunk_offsets[i * 4..i * 4 + 4];
            let chunk_location = chunk_location.read_u32::<BigEndian>()?;

            let offset = ((chunk_location >> 8) & 0xFFFFFF) * 4096;
            let size = (chunk_location & 0xFF) * 4096;

            if size == 0 {
                continue;
            }

            let mut chunk_buf = vec![0u8; size as usize];
            file.seek(SeekFrom::Start(offset as u64))?;
            file.read_exact(&mut chunk_buf)?;

            let chunk = Chunk::read(i, &mut chunk_buf)?;
            chunks.push(chunk);
        }

        Ok(Region { chunks })
    }
}
