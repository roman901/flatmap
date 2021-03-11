pub mod block;
pub mod chunk;
pub mod region;
pub mod section;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnvilError {
    #[error(transparent)]
    ReadError(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] nbt::Error),
    #[error("Found empty region file")]
    EmptyRegionError(),
    #[error("Found unsupported chunk compression algorithm")]
    UnsupportedChunkCompression(),
}
