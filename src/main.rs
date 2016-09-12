extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

mod gui;
mod rom;

fn main() {
  // gui::run();
  let rom = rom::Rom::new("sonic.bin");
  println!("|{}|", rom.system_name().unwrap());
}
