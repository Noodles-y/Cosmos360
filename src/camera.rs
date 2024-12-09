use cgmath;
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug)]
pub struct PolarCoordinate {
    angular: f32,//in degrees from 0 to 360
    radial: f32,//in degrees from -90 to 90
}


pub struct Camera {
    coordinates: PolarCoordinate,
    radial_max_range: f32,
    pub eye: cgmath::Point3<f32>,
    //pub target: cgmath::Point3<f32>,
    pub target: cgmath::Vector3<f32>,
    pub up: cgmath::Vector3<f32>,
    aspect: f32,
    fovx: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {

    pub fn new(screen: PhysicalSize<u32>) -> Self {

        println!("Aspect ratio : {} / {} = {}",
            screen.width as f32,
            screen.height as f32,
            screen.width as f32 / screen.height as f32
            );

        Self {
            coordinates: PolarCoordinate{angular:90.0,radial:45.0},
            radial_max_range: 80.0, 
            eye: (0.0, 0.0, 0.0).into(),
            target: (0.0, 0.0, 1.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: screen.width as f32 / screen.height as f32,
            fovx: 142.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        use cgmath::{Vector3, InnerSpace};
        //let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        //let view = cgmath::Matrix4::look_at_rh(self.target, self.eye, self.up);

        let fovx_rad = self.fovx.to_radians();
        let fovy_rad = 2.0 * ((0.5 * fovx_rad).tan() / self.aspect).atan();
        let fovy = fovy_rad.to_degrees();
        
        let up = self.target.cross(Vector3::unit_x()).normalize();

        let view = cgmath::Matrix4::look_to_rh(self.eye, self.target, up);
        let proj = cgmath::perspective(cgmath::Deg(fovy), self.aspect, self.znear, self.zfar);

        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        
        println!("current dir : {:?} current up :  {:?}", self.target, up);
        //println!("current fovx : {} fovy : {}", self.fovx, fovy);
        //println!("Matrix View : {:?}", view);
        //println!("Matrix Proj : {:?}", proj);
        //println!("Matrix VwPj : {:?}", view_proj);

        return view_proj;
        //return proj * view;
    }

    pub fn orientation(&self) -> &PolarCoordinate {
        &self.coordinates
    }

    pub fn set_orientation(&mut self, r: u32, a: u32) {
        todo!()
    }

    pub fn rotate(&mut self, angular_delta: f32, radial_delta: f32) {
        use cgmath::{InnerSpace, Deg, Matrix3, Quaternion, Rotation3, Rotation, Vector3};

        self.coordinates.angular += angular_delta;
        self.coordinates.radial += radial_delta;

        if self.coordinates.angular < 0.0 { self.coordinates.angular += 360.0;}
        if self.coordinates.angular > 360.0 { self.coordinates.angular -= 360.0;}
        if self.coordinates.radial < -self.radial_max_range { self.coordinates.radial = -self.radial_max_range;}
        if self.coordinates.radial > self.radial_max_range { self.coordinates.radial = self.radial_max_range;}


        //let forward = (self.target - self.eye).normalize();


        //let angular_angle = Deg(self.coordinates.angular);
        //let rot_matrix = Matrix3::from_angle_y(angle);
        //let current_forward = rot_matrix * forward;
        //let angular_rotation = Quaternion::from_axis_angle(Vector3::unit_y(), angular_angle);
        //let current_forward = angular_rotation.rotate_vector(forward);
        //let current_forward = rot_matrix * forward;

        //let right = current_forward.cross(self.up);
        //let radial_angle = Deg(self.coordinates.radial);
        //let rot_matrix = Matrix3::from_axis_angle(right, radial_angle);
        //let current_forward = rot_matrix * current_forward;
        //let radial_rotation = Quaternion::from_axis_angle(right, radial_angle);
        //let current_forward = radial_rotation.rotate_vector(current_forward); 
        //self.target = current_forward;

        let radial_rotation = Quaternion::from_axis_angle(Vector3::unit_x(), Deg(self.coordinates.radial));
        let angular_rotation = Quaternion::from_axis_angle(Vector3::unit_y(), Deg(self.coordinates.angular));
        let combined_rotation = angular_rotation * radial_rotation;

        self.target = combined_rotation.rotate_vector(Vector3::unit_z()).normalize();

        //self.target = Vector3::<f32>::new(1.0, -0.5, 0.0).normalize();

        //exemple si on regarde au centre de l'image
        //self.target = cgmath::Vector3::unit_x();

        println!("current coordinates : {:?} target: {:?}", self.coordinates, self.target);
        //println!("current right : {:?}", right);
    }

    pub fn change_fov(&mut self, delta_fov: f32) {
        self.fovx += delta_fov;
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
    angular: f32,
    radial: f32,
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            angular: 0.0,
            radial: 0.0,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
        self.angular = camera.orientation().angular;
        self.radial = camera.orientation().radial;
    }
}

