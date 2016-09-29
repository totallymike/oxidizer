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

use system::System;

fn main() {
  // gui::run();
  let mut system = {
    let system = system::System::new();
    let rom = rom::Rom::new("sonic.bin").unwrap();

    System::load_rom(system, rom)
  };
  
  system.cpu.pc_register = 0x206;
  let mut result = System::process_next(system);
  while let Ok(system) = result {
    result = System::process_next(system);
  }

  if let Err(err) = result {
    println!("{}", err);
  }

  // println!("|{}|", rom.system_name);
  // println!("|{}|", rom.domestic_name);
  // println!("{:#010x}", rom.rom_start);
  // println!("{:#010X}", rom.rom_end);
  // println!("|{}|", rom.control_data);

  // println!("|{:#018b}|", inst);
}
