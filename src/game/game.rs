use glutin;
use gfx;
use gfx::Device;
use gfx::handle::{RenderTargetView, DepthStencilView};
use gfx_window_glutin;
use device_gl;

use render::{ColorFormat, DepthFormat};


const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];


pub struct Game<'a> {
    //assets: AssetManager<'b>,
    //state_machine: StateMachine,
    //planner: Planner<()>,
    //start_instant: time::Instant,
    //last_update: time::Instant,
    //accumulated_delta: time::Duration,
    //time_per_frame: time::Duration,
    //frame_counter: u32, // 在每秒100帧的情况下，连续运行约500天将溢出
    window: glutin::Window,
    device: device_gl::Device,
    factory: device_gl::Factory,
    events_loop: &'a glutin::EventsLoop,
    rtv: RenderTargetView<device_gl::Resources, ColorFormat>,
    dsv: DepthStencilView<device_gl::Resources, DepthFormat>,
    running: bool,
}

impl<'a> Game<'a> {
    pub fn start(title: &str, win_w: u32, win_h: u32) {
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(win_w, win_h)
            .with_vsync();
        let events_loop = glutin::EventsLoop::new();
        let (window, device, factory, rtv, dsv) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

        let mut game = Game {
            window,
            device,
            factory,
            events_loop: &events_loop,
            rtv,
            dsv,
            running: true,
        };

        game.run();
    }

    fn run(&mut self) {
        let mut encoder: gfx::Encoder<_, _> = self.factory.create_command_buffer().into();
        while self.running {
            self.events_loop.poll_events(|glutin::Event::WindowEvent {
                 event, ..
             }| {
                use glutin::WindowEvent::*;
                use glutin::VirtualKeyCode;
                match event {
                    KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _) |
                    Closed => {
                        self.running = false;
                    }
                    Resized(_, _) => {
                        gfx_window_glutin::update_views(&self.window, &mut self.rtv, &mut self.dsv);
                    }
                    _ => {}
                }
            });
            encoder.clear(&self.rtv, BLACK);
            // draw here
            encoder.flush(&mut self.device);
            self.window.swap_buffers().unwrap();
            self.device.cleanup();
        }
    }
}
