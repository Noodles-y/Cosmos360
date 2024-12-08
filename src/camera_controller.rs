use winit::event::{
    WindowEvent,
    KeyEvent,
    ElementState,
};

use winit::keyboard::{
    KeyCode,
    PhysicalKey,
};

use crate::camera::Camera;

pub struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                //let is_pressed = *state == ElementState::Pressed;// bizarre, on devrait utiliser
                                                                 // state.is_pressed()
                let is_pressed = state.is_pressed();
                match keycode {KeyCode::KeyW | KeyCode::ArrowUp => {
                        println!("Press key Up : {}", is_pressed);
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        println!("Press key Left : {}", is_pressed);
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        println!("Press key Down");
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        println!("Press key Right");
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let (mut angular_delta, mut radial_delta) = (0.0, 0.0);

        if self.is_right_pressed {
            angular_delta += 1.0;
        }
        if self.is_left_pressed {
            angular_delta -= 1.0;
        }
        if self.is_up_pressed {
            radial_delta += 1.0;
        }
        if self.is_down_pressed {
            radial_delta -= 1.0;
        }

        angular_delta *= self.speed;
        radial_delta *= self.speed;

        camera.rotate(angular_delta, radial_delta);

    }
}

 

 
