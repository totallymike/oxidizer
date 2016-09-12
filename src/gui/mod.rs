use glutin;
use gfx_core::format::{DepthStencil, Rgba8};
use gfx_window_glutin;

pub fn run() {
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
