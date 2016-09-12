use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::str;
use std::str::Utf8Error;

use std::path::Path;

pub struct Rom<'a> {
  filename: &'a Path,
}

impl<'a> Rom<'a> {
  pub fn new(path: &str) -> Rom {
    Rom {
      filename: Path::new(path)
    }
  }

  pub fn system_name(&self) -> io::Result<String> {
    let mut f = try!(File::open(self.filename));
    let mut buffer: [u8; 0xF] = [0; 0xF];
    try!(f.seek(SeekFrom::Start(0x100)));
    try!(f.read_exact(&mut buffer));
    let thing = str::from_utf8(&buffer).unwrap();
    Ok(thing.to_string())
  }
}

pub fn system_name() -> String {
  "Hello".to_string()
}
