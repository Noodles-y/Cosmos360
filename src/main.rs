mod application;
mod image_data;
mod state;
mod camera;
mod camera_controller;
mod texture;

use application::CosmosViewer;

pub fn main() -> Result<(), &'static str> {

    //let event_loop = EventLoop::new().unwrap();
    //event_loop.set_control_flow(ControlFlow::Poll);

    // load an image
    let mut cosmos_viewer = CosmosViewer::new();
    cosmos_viewer.load_image()?;
    
    // start the application
    let handler = CosmosViewer::run();

    let _ = handler.join();

    Ok(())
}
