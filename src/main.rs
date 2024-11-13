mod application;
mod image_data;
mod state;

use application::Application;
use winit::event_loop::{EventLoop, ControlFlow};

pub fn main() {
    let mut app = Application::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run_app(&mut app).unwrap();
}
