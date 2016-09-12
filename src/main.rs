extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx_core::format::{DepthStencil, Rgba8};

fn main() {
  let builder = glutin::WindowBuilder::new().with_title("Example".to_string());

  let (window, _, _, _, _) =
    gfx_window_glutin::init::<Rgba8, DepthStencil>(builder);

  for event in window.wait_events() {
    match event {
      glutin::Event::Closed => break,
      _ => ()
    }
  }
}
