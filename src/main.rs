mod application;
mod image_data;
mod state;

use application::Application;
use winit::event_loop::{EventLoop, ControlFlow};

pub fn main() {

    let event_loop = EventLoop::new().unwrap();
    //event_loop.set_control_flow(ControlFlow::Poll);

    // create the application
    let mut app = Application::new();

    // load the image
    //let filename = "../image.png";
    //app.load_image(filename);

    event_loop.run_app(&mut app).unwrap();
}
