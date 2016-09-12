extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

mod gui;
mod rom;

fn main() {
  // gui::run();
  let mut rom = rom::Rom::new("sonic.bin").unwrap();
  println!("|{}|", rom.system_name());
  println!("|{}|", rom.domestic_name());
  println!("{:#010x}", rom.rom_start());
  println!("{:#010X}", rom.rom_end());
}
