use crate::kaitai_stream::KaitaiStream;
use std::fs::File;
use std::io::{Cursor, Result};

pub trait KaitaiStruct {
    fn from_file(path: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let mut f = File::open(path)?;
        Self::new(&mut f, &None, &None)
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut b = Cursor::new(bytes);
        Self::new(&mut b, &None, &None)
    }

    fn new<S: KaitaiStream>(
        stream: &mut S,
        parent: &Option<Box<dyn KaitaiStruct>>,
        root: &Option<Box<dyn KaitaiStruct>>,
    ) -> Result<Self>
    where
        Self: Sized;

    fn read<S: KaitaiStream>(
        &mut self,
        stream: &mut S,
        parent: &Option<Box<dyn KaitaiStruct>>,
        root: &Option<Box<dyn KaitaiStruct>>,
    ) -> Result<()>
    where
        Self: Sized;
}
