use winit::event::{
    WindowEvent,
    KeyEvent,
};

use winit::keyboard::{
    KeyCode,
    PhysicalKey,
};

use crate::camera::{Camera, CameraSettingsBuffer};

pub struct CameraController {
    speed: f32,
    mouse_sensibility: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    increase_fov: bool,
    decrease_fov: bool,
}

impl CameraController {
    pub fn new(speed: f32, mouse_sensibility: f32) -> Self {
        Self {
            speed,
            mouse_sensibility,
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            increase_fov: false,
            decrease_fov: false,
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
                    KeyCode::KeyJ => {
                        println!("Increase Fov");
                        self.increase_fov = is_pressed;
                        true
                    }
                    KeyCode::KeyK => {
                        println!("Decrease Fov");
                        self.decrease_fov = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera, settings: &mut CameraSettingsBuffer) {
        let (mut angular_delta, mut radial_delta) = (0.0, 0.0);

        if self.is_right_pressed {
            angular_delta -= 1.0;
        }
        if self.is_left_pressed {
            angular_delta += 1.0;
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

        let zoom_factor = 
            if self.increase_fov {1.1}
            else if self.decrease_fov {0.9}
            else {1.0};
        settings.zoom(zoom_factor);
    }
    
    pub fn move_cursor(&self, camera: &mut Camera, delta_x: f64, delta_y: f64) {
        camera.rotate((-delta_x as f32) * self.mouse_sensibility, (-delta_y as f32) * self.mouse_sensibility);
    }
}

 

 
