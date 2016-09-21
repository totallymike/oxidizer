use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use byteorder::{BigEndian,ReadBytesExt,WriteBytesExt};

use rom;

pub struct System {
  pub memory: Box<[u8]>,
}

impl System {
  pub fn new() -> System {
    System {
      memory: vec![0; 0x1FFFFFE].into_boxed_slice(),
    }
  }

  pub fn load_rom(system: System, rom: rom::Rom) -> System {
    let mut fd = rom.fd;
    fd.seek(SeekFrom::Start(0)).expect("Couldn't rewind!");
    let mut cursor = Cursor::new(system.memory);
    cursor.seek(SeekFrom::Start(0)).expect("hrm...");
    while let Ok(n) = fd.read_u16::<BigEndian>() {
      cursor.write_u16::<BigEndian>(n).expect("Write failed!");
    }

    System { memory: cursor.into_inner() }
  }
}
