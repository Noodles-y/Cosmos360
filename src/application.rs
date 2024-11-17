use crate::state::State;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowId, WindowAttributes, Fullscreen},
};

pub struct Application {
    state: Option<State>,
}

impl Application {
    
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler for Application {


    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(WindowAttributes::default()
                .with_title("My first Rust window")
                .with_inner_size(winit::dpi::LogicalSize::new(1920.0, 1080.0))
                .with_fullscreen(Some(Fullscreen::Borderless(None)))
        ).unwrap();

        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.state.as_ref().unwrap().window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    println!("Closing window");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    //executor::block_on( run(self.window.as_ref().unwrap(), &self.image_data) );
                    //window.request_redraw();
                    self.state.as_mut().unwrap().render().unwrap();// use unwrap() to panic in case
                                                                   // of render fail
                }
                _ => (),
            }
        }
    }
}
