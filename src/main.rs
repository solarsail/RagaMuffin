#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, rtv, stv) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);
    'main: loop {
        for event in events_loop.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {}
            }
        }
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
