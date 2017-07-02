use glutin;
use glutin::WindowEvent::{KeyboardInput, Closed};
use glutin::Event::WindowEvent;
use glutin::{VirtualKeyCode, EventsLoop};
use gfx_window_glutin;
use device_gl;
use gfx::{format, Device};


pub type ColorFormat = format::Rgba8;
pub type DepthFormat = format::DepthStencil;

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
    events_loop: &'a EventsLoop,
    running: bool,
}

impl<'a> Game<'a> {
    pub fn start(title: &str, win_w: u32, win_h: u32) {
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(win_w, win_h)
            .with_vsync();
        let events_loop = glutin::EventsLoop::new();
        let (window, device, factory, _rtv, _stv) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

        let mut game = Game {
            window,
            device,
            factory,
            events_loop: &events_loop,
            running: true,
        };

        game.run();
    }

    fn run(&mut self) {
        while self.running {
            self.events_loop
                .poll_events(|e| match e {
                                 WindowEvent {
                                     event: KeyboardInput(_, _, Some(VirtualKeyCode::Escape), _), ..
                                 } |
                                 WindowEvent { event: Closed, .. } => {
                    self.running = false;
                }
                                 _ => {}
                             });
            self.window.swap_buffers().unwrap();
            self.device.cleanup();
        }
    }
}
