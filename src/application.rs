use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId, WindowAttributes},
};

#[derive(Default)]
pub struct Application {
    window : Option<Window>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(WindowAttributes::default()
                .with_title("My first Rust window")
                .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        ).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                //executor::block_on( run(self.window.as_ref().unwrap(), &self.image_data) );
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
