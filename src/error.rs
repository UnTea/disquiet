use std::io;
use png;
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("PNG encoding error: {0}")]
    PngEncodingError(#[from] png::EncodingError),

    #[error("PNG decoding error: {0}")]
    PngDecodingError(#[from] png::DecodingError),

    #[error("HDR decoding error")]
    HdrDecodingError,

    #[error("Parse error: {0}")]
    ParseIntError(#[from] ParseIntError)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
