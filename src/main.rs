#![feature(plugin)]
#![plugin(clippy)]
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

mod gui;
mod rom;
mod m68k;

mod system;

use byteorder::{BigEndian,ReadBytesExt};
use std::io::Cursor;
use std::io::prelude::*;
use std::io::SeekFrom;
use system::System;

fn main() {
  // gui::run();
  let system = {
    let system = system::System::new();
    let rom = rom::Rom::new("sonic.bin").unwrap();

    System::load_rom(system, rom)
  };
  
  println!("|{:#010b}|", system.memory[0x206]);  
  let mut my_cursor = Cursor::new(system.memory);

  my_cursor.seek(SeekFrom::Start(0x206)).expect("Couldn't seek!");
  println!("|{:#018b}|", my_cursor.read_u16::<BigEndian>().unwrap());
  println!("|{:#010x}|", my_cursor.read_u16::<BigEndian>().unwrap());
  println!("|{:#010x}|", my_cursor.read_u16::<BigEndian>().unwrap());
  // println!("|{}|", rom.system_name);
  // println!("|{}|", rom.domestic_name);
  // println!("{:#010x}", rom.rom_start);
  // println!("{:#010X}", rom.rom_end);
  // println!("|{}|", rom.control_data);
  // let mut rom = rom;
  // let inst = rom.first_instruction();
  // let opcode = m68k::OpCodes::from_bits_truncate(inst);
  // if opcode == m68k::TST_L {
  //   println!("Yep");
  // }
  // println!("|{:#018b}|", inst);
}
