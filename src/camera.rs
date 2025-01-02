use cgmath::{Point3, Matrix4, Vector3, InnerSpace, Quaternion, Rotation, Rotation3, Deg};
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
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
    pub eye: Point3<f32>,
    //pub target: cgmath::Point3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    aspect: f32,
    fovx: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {

    pub fn new(screen: PhysicalSize<u32>) -> Self {

        /*println!("Aspect ratio : {} / {} = {}",
            screen.width as f32,
            screen.height as f32,
            screen.width as f32 / screen.height as f32
            );
*/
        Self {
            coordinates: PolarCoordinate{angular:0.0,radial:0.0},
            radial_max_range: 90.0, 
            eye: (0.0, 0.0, 0.0).into(),
            target: (0.0, 0.0, 1.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: screen.width as f32 / screen.height as f32,
            fovx: 142.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn display_matrix(name: &str, matrix: Matrix4<f32>) {
        println!("{} :", name);
        println!("{:5.2} {:5.2} {:5.2} {:5.2}", matrix.x.x, matrix.x.y, matrix.x.z, matrix.x.w);
        println!("{:5.2} {:5.2} {:5.2} {:5.2}", matrix.y.x, matrix.y.y, matrix.y.z, matrix.y.w);
        println!("{:5.2} {:5.2} {:5.2} {:5.2}", matrix.z.x, matrix.z.y, matrix.z.z, matrix.z.w);
        println!("{:5.2} {:5.2} {:5.2} {:5.2}", matrix.w.x, matrix.w.y, matrix.w.z, matrix.w.w);
    }

    fn fovx_to_fovy(fovx: f32, aspect: f32) -> f32 {
        let fovx_rad = fovx.to_radians();
        let fovy_rad = 2.0 * ((0.5 * fovx_rad).tan() / aspect).atan();
        fovy_rad.to_degrees()
    }

    pub fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_to_lh(self.eye, self.target, self.up);
        Self::display_matrix("view", view);
        view
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        //use cgmath::{Vector3, InnerSpace};
        //let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        //let view = cgmath::Matrix4::look_at_rh(self.target, self.eye, self.up);

        //let fovx_rad = self.fovx.to_radians();
        //let fovy_rad = 2.0 * ((0.5 * fovx_rad).tan() / self.aspect).atan();
        //let fovy = fovy_rad.to_degrees();
        let fovy = Self::fovx_to_fovy(self.fovx, self.aspect);

        // test
        let target = self.target;
        let up = self.up;
        //let target = -Vector3::unit_y();
        //let up = -Vector3::unit_x();

        let view = cgmath::Matrix4::look_to_lh(self.eye, target, up);
        //let view = cgmath::Matrix4::look_to_rh(self.eye, -Vector3::unit_y(), -Vector3::unit_z());
        Self::display_matrix("east-down target", cgmath::Matrix4::look_to_lh(self.eye, -Vector3::unit_y(), -Vector3::unit_z()));
        
        let proj = cgmath::perspective(Deg(fovy), self.aspect, self.znear, self.zfar);
        
        Self::display_matrix("view", view);

        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        //let view_proj = OPENGL_TO_WGPU_MATRIX * view;
        
        println!("coordinates : angular = {:.0} radial = {:.0} ", self.coordinates.angular, self.coordinates.radial);
        println!("target : [{:.2}, {:.2}, {:.2}] up :  [{:.2}, {:.2}, {:.2}]",
            self.target.x, self.target.y, self.target.z,
            self.up.x, self.up.y, self.up.z,
            );
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

    pub fn rotate(&mut self, angular_delta: f32, radial_delta: f32) {
        //use cgmath::{InnerSpace, Deg, Quaternion, Rotation3, Rotation, Vector3};

        self.coordinates.angular += angular_delta;
        self.coordinates.radial += radial_delta;

        // normalize angular coordinates [0;360]
        self.coordinates.angular = (self.coordinates.angular % 360.0 + 360.0) % 360.0;
        // normalize radial coordinates [-max_range;max_range]
        self.coordinates.radial = self.coordinates.radial.clamp(-self.radial_max_range, self.radial_max_range);
        
        //self.target = Self::compute_target(self.coordinates.angular, self.coordinates.radial);
        //self.up = Self::compute_up(self.target, self.coordinates.angular);

        
        let right = Quaternion::from_axis_angle(Vector3::unit_y(), Deg((self.coordinates.angular + 90.0) % 360.0))
            .rotate_vector(Vector3::unit_z());
        // compute target vector
        let radial_rotation = Quaternion::from_axis_angle(Vector3::unit_x(), Deg(self.coordinates.radial));
        let angular_rotation = Quaternion::from_axis_angle(Vector3::unit_y(), Deg(self.coordinates.angular));

        self.target = radial_rotation.rotate_vector(Vector3::unit_z()).normalize();
        self.target = angular_rotation.rotate_vector(self.target).normalize();

        // compute up vector
        self.up = self.target.cross(right).normalize();
        
        //println!("current coordinates : {:?} right : {:?}", self.coordinates, right);
        //println!("cross product magnitude : {}", self.target.cross(right).magnitude());
    }

    pub fn change_fov(&mut self, delta_fov: f32) {
        let new_fovy = Self::fovx_to_fovy(self.fovx+delta_fov, self.aspect); 

        if new_fovy > 0.0 && new_fovy < 360.0 {
            self.fovx += delta_fov;
        }

        /*println!("current fovx = {} fovy = {}, new_fovy = {}", 
            self.fovx, 
            Self::fovx_to_fovy(self.fovx, self.aspect),
            new_fovy,
            );*/
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

// *** Tests ***

fn spheric_to_equirectangular(longitude: f32, latitude: f32) -> cgmath::Point2<f32> {
    // center of the map (longitude, latitude)
    let origin = cgmath::Point2::<f32>::new(0.0, 0.0);
    let x = (origin.y.to_radians().cos() * (longitude - origin.x))/360.0;
    let y = (latitude - origin.y)/180.0;

    println!("spheric to equirectangular : longitude = {:.0} latitude: {:.0}", longitude, latitude);
    println!("X = ( cos({:.1} PI) * ({:.0} - {:.0}) ) / 360", origin.y.to_radians()/3.141592, longitude, origin.x);
    println!("Y = ({} - {} ) / 180", latitude, origin.y);
    println!("X = {:.2} Y = {:.2}", x, y);

    cgmath::Point2::<f32>::new(x, y)
}

fn spheric_to_cartesian(azimuth_deg: f32, elevation_deg: f32) -> cgmath::Point2<f32> {
    use cgmath::Point2;

    let azimuth_rad = azimuth_deg.to_radians();
    let elevation_rad = elevation_deg.to_radians();
    let mut result = Point2::new(0.0, 0.0);

    result.x = elevation_rad.sin() * azimuth_rad.cos();
    result.y = elevation_rad.cos();

    println!("spheric_to_cartesian : azimuth = {}° ({:.1} PI rad) elevation = {}° ({:.1} PI rad)",
        azimuth_deg,
        azimuth_rad / 3.141592,
        elevation_deg,
        elevation_rad / 3.141592,
        );
    //println!("sin(azimuth) = {:.2} cos(azimuth) = {:.2} sin(elevation) = {:.2} cos(elevation) = {:.2}",
    //    elevation_rad.sin(), azimuth_rad.cos(), elevation_rad.sin(), elevation_rad.cos());
    println!("X = sin({:.2}) x cos({:.2}) Y = cos({:.2}))"
        , elevation_rad, azimuth_rad
        , elevation_rad
        );
    println!("X = {:.2} x {:.2} Y = {:.2}"
        , elevation_rad.sin(), azimuth_rad.cos()
        , elevation_rad.cos()
        );
    println!("X = {:.2} Y = {:.2}", result.x, result.y);

    result
}

#[cfg(test)]
mod test_rotation {
    use super::*;
    use cgmath::{Vector3, Vector4};

    fn compare_f32(a: f32, b:f32) -> bool {
        let float_precision = 0.001;

        println!("{} == {}", a, b);

        (a - b).abs() < float_precision
    }

    fn compare_vector3(u: Vector3<f32>, v: Vector3<f32>) -> bool {
        println!("[{:.2}, {:.2}, {:.2}] == [{:.2}, {:.2}, {:.2}]",
            u.x, u.y, u.z,
            v.x, v.y, v.z);

        compare_f32(u.x, v.x) && compare_f32(u.y, v.y) && compare_f32(u.z, v.z)
    }

    fn camera_rotation(azimuth: f32, elevation: f32) -> Vector3<f32> {
        let mut camera = Camera::new(PhysicalSize::new(1920, 1080));
        camera.rotate(azimuth, elevation);
        println!("camera.target = [{:.2}, {:.2}, {:.2}] camera.up = [{:.2}, {:.2}, {:.2}]",
            camera.target.x, camera.target.y, camera.target.z,
            camera.up.x, camera.up.y, camera.up.z,
        );
        let view = camera.get_view_matrix();
        let target: Vector3<f32> = (view * Vector4::unit_z()).truncate();
        target
    }

    #[test]
    fn azimuth_0_elevation_90() {
        let target = camera_rotation(0.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn azimuth_90_elevation_90() {
        let target = camera_rotation(90.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(-1.0, 0.0, 0.0)));
    }
    
    #[test]
    fn azimuth_180_elevation_90() {
        let target = camera_rotation(180.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn azimuth_270_elevation_90() {
        let target = camera_rotation(270.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn azimuth_0_elevation_0() {
        let target = camera_rotation(0.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn azimuth_90_elevation_0() {
        let target = camera_rotation(90.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn azimuth_180_elevation_0() {
        let target = camera_rotation(180.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn azimuth_270_elevation_0() {
        let target = camera_rotation(270.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

}

#[cfg(test)]
mod test_spheric_to_equirectangular {
    use super::*;

    fn compare(a: f32, b:f32) -> bool {
        let float_precision = 0.001;

        (a - b).abs() < float_precision
    }

    #[test]
    fn azimuth_0_elevation_0() {
        let result = spheric_to_equirectangular(0.0, 0.0); 
        assert!(compare(result.x, 0.0));
        assert!(compare(result.y, 0.0));
    }
    
    #[test]
    fn azimuth_0_elevation_90() {
        let result = spheric_to_equirectangular(0.0, 90.0); 
        assert!(compare(result.x, 0.0));
        assert!(compare(result.y, 0.5));
    }

    #[test]
    fn azimuth_0_elevation_180() {
        let result = spheric_to_equirectangular(0.0, 180.0); 
        assert!(compare(result.x, 0.0));
        assert!(compare(result.y, 1.0));
    }

    #[test]
    fn azimuth_90_elevation_90() {
        let result = spheric_to_equirectangular(90.0, 90.0); 
        assert!(compare(result.x, 0.25));
        assert!(compare(result.y, 0.5));
    }
    
    #[test]
    fn azimuth_180_elevation_90() {
        let result = spheric_to_equirectangular(180.0, 90.0); 
        assert!(compare(result.x, 0.5));
        assert!(compare(result.y, 0.5));
    }
    
    #[test]
    fn azimuth_270_elevation_90() {
        let result = spheric_to_equirectangular(270.0, 90.0); 
        assert!(compare(result.x, 0.75));
        assert!(compare(result.y, 0.5));
    }
    
    #[test]
    fn azimuth_360_elevation_90() {
        let result = spheric_to_equirectangular(360.0, 90.0); 
        assert!(compare(result.x, 1.0));
        assert!(compare(result.y, 0.5));
    }
    
    #[test]
    fn azimuth_180_elevation_0() {
        let result = spheric_to_equirectangular(180.0, 0.0); 
        assert!(compare(result.x, 0.5));
        assert!(compare(result.y, 0.0));
    }
    
}
