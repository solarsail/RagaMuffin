#[macro_use]
extern crate gfx;
extern crate gfx_device_gl as device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate serde;
extern crate serde_derive;

use device_gl::Resources as R;
use render::ColorFormat;

pub type SRV = gfx::handle::ShaderResourceView<R, ColorFormat::View>;

mod game;
mod render;
mod geometry;

use game::Game;

pub fn main() {
    Game::start("test title", 800, 600);
}
