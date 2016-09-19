#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

mod gui;
mod rom;
mod m68k;

fn main() {
  // gui::run();
  let rom = rom::Rom::new("sonic.bin").unwrap();
  println!("|{}|", rom.system_name);
  println!("|{}|", rom.domestic_name);
  println!("{:#010x}", rom.rom_start);
  println!("{:#010X}", rom.rom_end);
  println!("|{}|", rom.control_data);
  let mut rom = rom;
  let inst = rom.first_instruction();
  let opcode = m68k::OpCodes::from_bits_truncate(inst);
  if opcode == m68k::TST_L {
    println!("Yep");
  }
  println!("|{:#018b}|", inst);
}
