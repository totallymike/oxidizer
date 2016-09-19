use byteorder::{BigEndian, ReadBytesExt};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::BufReader;
use std::mem;
use std::str;
use std::borrow::Cow;

use std::path::Path;

pub struct Rom {
  pub system_name: String,
  pub domestic_name: String,
  pub control_data: String,
  pub rom_start: u32,
  pub rom_end: u32,
  rom_data: Vec<u8>,
  fd: File,
}

impl Rom {
  pub fn new(path: &str) -> io::Result<Rom> {
    let path = Path::new(path);
    let mut fd = try!(File::open(path));

    let raw_data = rom_data(&mut fd);
    Ok(Rom {
      system_name: system_name(&mut fd),
      domestic_name: domestic_name(&mut fd),
      control_data: control_data(&mut fd),
      rom_start: rom_start(&mut fd),
      rom_end: rom_end(&mut fd),
      rom_data: raw_data,
      fd: fd,
    })
  }
  pub fn first_instruction(&mut self) -> u16 {
    let mut fd = &self.fd;
    fd.seek(SeekFrom::Start(0x206)).expect("Couldn't seek!");
    fd.read_u16::<BigEndian>().unwrap()
  }
}

fn rom_data(fd: &mut File) -> Vec<u8> {
  fd.seek(SeekFrom::Start(0)).expect("Couldn't rewind!");
  let mut buffer: Vec<u8> = Vec::new();
  fd.read_to_end(&mut buffer).expect("D'oh");
  buffer.shrink_to_fit();
  buffer
}
fn system_name<'a>(fd: &mut File) -> String {
  let data = read_string(fd, 0x100, 0xF).unwrap();
  data.trim().to_owned()
}

fn domestic_name(fd: &mut File) -> String {
  let data = read_string(fd, 0x120, 0x2F).unwrap();

  data
    .split_whitespace()
    .collect::<Vec<&str>>()
    .join(" ")
}

fn control_data(fd: &mut File) -> String {
  let data = read_string(fd, 0x190, 0x10).unwrap();
  data
    .trim()
    .to_owned()
}

fn rom_start(fd: &mut File) -> u32 {
  let data = read_bytes(fd, 0x1A0, 4).unwrap();
  let mut dst: [u8; 4] = [0; 4];
  dst.clone_from_slice(data.as_slice());
  let data = dst;
  unsafe {
    let raw = mem::transmute::<[u8; 4], u32>(data);
    u32::from_be(raw)
  }
}

fn rom_end(fd: &mut File) -> u32 {
  let data = read_bytes(fd, 0x1A4, 4).unwrap();
  let mut dst: [u8; 4] = [0; 4];
  dst.clone_from_slice(data.as_slice());
  let data = dst;
  unsafe {
    let raw = mem::transmute::<[u8; 4], u32>(data);
    u32::from_be(raw)
  }
}

fn read_bytes(fd: &mut File, start: u64, length: usize) -> io::Result<Vec<u8>> {
  try!(fd.seek(SeekFrom::Start(start)));
  let mut handle = fd.take(length as u64);
  let mut buffer: Vec<u8> = Vec::with_capacity(length);

  try!(handle.read_to_end(&mut buffer));
  Ok(buffer)
}

fn read_string(fd: &mut File, start: u64, length: usize) -> io::Result<String> {
  let buffer = try!(read_bytes(fd, start, length));
  Ok(String::from_utf8(buffer).unwrap())
}
