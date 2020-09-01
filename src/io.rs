pub mod hdr;

use crate::error::*;
use std::path::Path;

pub trait Load : Sized {
    fn load<P: AsRef<Path>>(p: P) -> Result<Self>;
}

pub fn load<L: Load, P: AsRef<Path>>(p: P) -> Result<L> {
    L::load(p)
}
