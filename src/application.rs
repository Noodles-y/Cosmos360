use crate::state::State;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowId, WindowAttributes},
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
                .with_title("Cosmos360")
                .with_inner_size(winit::dpi::LogicalSize::new(1920.0, 1080.0))
                //.with_fullscreen(Some(Fullscreen::Borderless(None)))
        ).unwrap();

        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        //let window = state.window();

        if state.window().id() == window_id {
            
            if state.input(&event) {
                state.window().request_redraw();
            }
            else {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Closing window");
                        event_loop.exit();
                    }
                    //WindowEvent::KeyboardInput => {
                    //    self.state.as_mut().unwrap().input(&event);
                    //}
                    WindowEvent::RedrawRequested => {
                        //executor::block_on( run(self.window.as_ref().unwrap(), &self.image_data) );
                        //window.request_redraw();

                        self.state.as_mut().unwrap().update();

                        self.state.as_mut().unwrap().render().unwrap();// use unwrap() to panic in case
                                                                       // of render fail
                    }
                    WindowEvent::Resized(new_size) => {
                        self.state.as_mut().unwrap().resize(new_size);
                    }
                    _ => (),
                }
            }
        }
    }
}
