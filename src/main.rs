#[macro_use]
extern crate gfx;
extern crate gfx_device_gl as device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

mod game;

use game::Game;

pub fn main() {
    Game::start("test title", 800, 600);
}
