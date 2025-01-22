use cgmath::{Matrix4, Vector3, Deg};
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct PolarCoordinate {
    angular: f32,//in degrees from 0 to 360
    radial: f32,//in degrees from -90 to 90
}

pub struct Camera {
    coordinates: PolarCoordinate,
    radial_max_range: f32,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {

    pub fn new(_screen: PhysicalSize<u32>) -> Self {
        Self {
            coordinates: PolarCoordinate{angular:0.0,radial:90.0},
            radial_max_range: 90.0, 
            target: (0.0, 0.0, 1.0).into(),
            up: cgmath::Vector3::unit_y(),
        }
    }

    pub fn rotate(&mut self, angular_delta: f32, radial_delta: f32) {
        
        self.move_coordinates(angular_delta, radial_delta);
        self.target = (self.rotation_matrix() * cgmath::Vector4::unit_z()).truncate();
        self.up = (self.rotation_matrix() * cgmath::Vector4::unit_y()).truncate();
        
        println!("coordinates : angular = {:.0} radial = {:.0} ", self.coordinates.angular, self.coordinates.radial);
        println!("target : [{:.2}, {:.2}, {:.2}] up :  [{:.2}, {:.2}, {:.2}]",
            self.target.x, self.target.y, self.target.z,
            self.up.x, self.up.y, self.up.z,
            );
    }

    fn move_coordinates(&mut self, angular_delta: f32, radial_delta: f32) {
        self.coordinates.angular += angular_delta;
        self.coordinates.radial += radial_delta;

        // normalize angular coordinates [0;360]
        self.coordinates.angular = (self.coordinates.angular % 360.0 + 360.0) % 360.0;
        // normalize radial coordinates [-max_range;max_range]
        self.coordinates.radial = self.coordinates.radial.clamp(90.0-self.radial_max_range, 90.0+self.radial_max_range);
    }

    pub fn rotation_matrix(&self) -> Matrix4<f32> {
        let rotation_elevation = Matrix4::<f32>::from_angle_x(Deg(90.0 - self.coordinates.radial));
        let rotation_azimuth = Matrix4::<f32>::from_angle_y(Deg(-self.coordinates.angular));

        rotation_azimuth * rotation_elevation
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.rotation_matrix().into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraSettingsBuffer {
    width: f32,
    height: f32,
    focal_length: f32,
}

impl CameraSettingsBuffer {
    pub fn new() -> Self {
        Self {
            width: 2.0,
            height: 2.0,
            focal_length: 1.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn zoom(&mut self, multiplier: f32) {
        self.focal_length = (self.focal_length * multiplier).clamp(0.001,1000.0);
    }
}
