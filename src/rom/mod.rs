use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::mem;
use std::str;

use std::path::Path;

pub struct Rom {
  fd: File,
}

impl Rom {
  pub fn new(path: &str) -> io::Result<Rom> {
    let path = Path::new(path);
    let fd = try!(File::open(path));
    Ok(Rom {
      fd: fd,
    })
  }

  pub fn system_name(&mut self) -> String {
    let data = self.read_string(0x100, 0xF).unwrap();

    data
      .trim()
      .to_owned()
  }

  pub fn domestic_name(&mut self) -> String {
    let data = self.read_string(0x120, 0x2F).unwrap();

    data
      .split_whitespace()
      .collect::<Vec<&str>>()
      .join(" ")
  }

  pub fn rom_start(&mut self) -> u32 {
    let data = self.read_bytes(0x1A0, 4).unwrap();
    let mut dst: [u8; 4] = [0; 4];
    dst.clone_from_slice(data.as_slice());
    let data = dst;
    unsafe {
      let raw = mem::transmute::<[u8; 4], u32>(data);
      u32::from_be(raw)
    }
  }

  pub fn rom_end(&mut self) -> u32 {
    let data = self.read_bytes(0x1A4, 4).unwrap();
    let mut dst: [u8; 4] = [0; 4];
    dst.clone_from_slice(data.as_slice());
    let data = dst;
    unsafe {
      let raw = mem::transmute::<[u8; 4], u32>(data);
      u32::from_be(raw)
    }
  }

  pub fn rom_instructions(&mut self) -> Vec<u8> {
    let mut f = &self.fd;
    let mut buffer: Vec<u8> = Vec::new();
    f.seek(SeekFrom::Start(0x200)).expect("Uh oh");
    f.read_to_end(&mut buffer).expect("didn't read");
    buffer
  }
  fn read_bytes(&mut self, start: u64, length: usize) -> io::Result<Vec<u8>> {
    let mut f = &self.fd;
    let mut handle = f.take(length as u64);
    let mut buffer: Vec<u8> = Vec::with_capacity(length);

    try!(f.seek(SeekFrom::Start(start)));
    try!(handle.read_to_end(&mut buffer));
    Ok(buffer)
  }

  fn read_string(&mut self, start: u64, length: usize) -> io::Result<String> {
    let buffer = try!(self.read_bytes(start, length));
    Ok(String::from_utf8(buffer).unwrap())
  }
}
