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
        Self {
            coordinates: PolarCoordinate{angular:0.0,radial:90.0},
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

    pub fn fovx_to_fovy(fovx: f32, aspect: f32) -> f32 {
        let fovx_rad = fovx.to_radians();
        let fovy_rad = 2.0 * ((0.5 * fovx_rad).tan() / aspect).atan();
        fovy_rad.to_degrees()
    }

    pub fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_to_lh(self.eye, self.target, self.up);
        Self::display_matrix("view", view);
        view
    }


    pub fn orientation(&self) -> &PolarCoordinate {
        &self.coordinates
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

    pub fn rotate_vector(vector: Vector3<f32>, azimuth: f32, elevation: f32) -> Vector3<f32> {
        let mut rotated_vector = vector;
        rotated_vector = Quaternion::from_axis_angle(Vector3::unit_x(), Deg(elevation))
            .rotate_vector(rotated_vector);
        rotated_vector = Quaternion::from_axis_angle(Vector3::unit_y(), Deg(azimuth))
            .rotate_vector(rotated_vector);
        rotated_vector
    }

    //pub fn perspective_matrix(fovx: f32, aspect: f32) -> Matrix4<f32> {
    pub fn perspective_matrix(&self) -> Matrix4<f32> {
        let fovy = Self::fovx_to_fovy(self.fovx, self.aspect);
        cgmath::perspective(Deg(fovy), self.aspect, self.znear, self.zfar)
    }

    //pub fn rotation_matrix(azimuth_deg: f32, elevation_deg: f32) -> Matrix4<f32> {
    pub fn rotation_matrix(&self) -> Matrix4<f32> {
        let rotation_elevation = Matrix4::<f32>::from_angle_x(Deg(90.0 - self.coordinates.radial));
        let rotation_azimuth = Matrix4::<f32>::from_angle_y(Deg(-self.coordinates.angular));

        rotation_azimuth * rotation_elevation
    }

    pub fn rotate2(&mut self, azimuth_delta: f32, elevation_delta: f32) {
        self.move_coordinates(azimuth_delta, elevation_delta);
        
        //self.target = Self::rotate_vector(Vector3::unit_y(), self.coordinates.angular, self.coordinates.radial);
        //self.up = Self::rotate_vector(-Vector3::unit_z(), self.coordinates.angular, self.coordinates.radial);

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

    pub fn set_fovx(&mut self, value: f32) {
        let new_fovy = Self::fovx_to_fovy(value, self.aspect); 

        if new_fovy > 0.0 && new_fovy < 360.0 {
            self.fovx = value;
        }
        else
        {
            println!("Camera::set_fovx() : Invalid value (fovx={:.2} => fovy={:.2} [0.0 ; 360.0])", value, new_fovy);
        }
    }


    pub fn get_fov(&self) -> (f32, f32) {
        (self.fovx, Self::fovx_to_fovy(self.fovx, self.aspect))
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
    fovx: f32,
    fovy: f32,
    azimuth: f32,
    elevation: f32,
}

impl CameraUniform {
    //use Camera::get_rotation_matrix;
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            fovx: 1.0,
            fovy: 1.0,
            azimuth: 0.0,
            elevation: 0.0,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        //self.view_proj = (camera.perspective_matrix() * camera.rotation_matrix()).into();
        self.view_proj = camera.rotation_matrix().into();
        (self.fovx, self.fovy) = camera.get_fov();
        self.azimuth = camera.orientation().angular;
        self.elevation = camera.orientation().radial;
    }
}

// *** Tests ***
/*
 *  screen_coords : (-1.0, -1.0) to (1.0, 1.0)
 *  camera_azimuth : 0.0 to 360.0 (azimuth of the center the camera)
 *  camera_elevation : 0.0 to 180.0 from north pole (azimuth of the center of the camera)
*/
pub fn screen_to_spheric(screen_coords: cgmath::Point2<f32>, camera_azimuth: f32, camera_elevation: f32, fovx: f32, aspect: f32) -> (f32, f32) {

    let fovy = Camera::fovx_to_fovy(fovx, aspect);

    println!("screen = [{:.2};{:.2}] camera = [a={:.0};e={:.0}] fovx = {:.3} fovy = {:.3}",
        screen_coords.x, screen_coords.y,
        camera_azimuth, camera_elevation,
        fovx, fovy);

    let (result_elevation, latitude) = screen_elevation(camera_elevation, fovy, screen_coords.y);
    let result_azimuth = screen_azimuth(camera_azimuth, fovx, screen_coords.x, latitude);


    (result_azimuth, result_elevation)
}

pub fn screen_elevation(camera_elevation: f32, fovy: f32, screen_coord_y: f32) -> (f32, f32) {

    let sy = 0.5 * fovy * screen_coord_y;
    let latitude = 90.0 - camera_elevation - sy;
    let elevation = 90.0 - latitude;

    println!("sy = {:.2} latitude = {:.2} elevation = {:.2}", sy, latitude, elevation);
    
    (elevation, latitude)
}

pub fn screen_azimuth(camera_azimuth: f32, fovx: f32, screen_coord_x: f32, latitude: f32) -> f32 {
    let sx = -0.5 * fovx * screen_coord_x;
    let cz = latitude.to_radians().cos();
    let azimuth = (camera_azimuth + sx.to_radians().atan2(cz).to_degrees() + 360.0) % 360.0;

    println!("atan2(cz, sx) = {} atan2(sx, cz) = {}", cz.atan2(sx).to_degrees(), sx.atan2(cz).to_degrees());
    println!("sx = {:.2} cz = {:.2} azimuth = {:.2}", sx, cz.to_degrees(), azimuth);

    azimuth
}

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

fn cartesian_to_spheric(vector: Vector3<f32>) -> (f32, f32) {
    let radius = (vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0)).sqrt();
    let azimuth = (vector.y / radius).acos().to_degrees();
    let elevation = vector.x.atan2(vector.z).to_degrees();
    
    println!("radius = {:.2} azimuth = {:.2}, elevation = {:.2}", radius, azimuth, elevation);

    (azimuth, elevation)
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
        let float_precision = 1.0;

        println!("{} == {}", a, b);

        (a - b).abs() < float_precision
    }

    fn compare_vector3(u: Vector3<f32>, v: Vector3<f32>) -> bool {
        println!("[{:.2}, {:.2}, {:.2}] == [{:.2}, {:.2}, {:.2}]",
            u.x, u.y, u.z,
            v.x, v.y, v.z);

        compare_f32(u.x, v.x) && compare_f32(u.y, v.y) && compare_f32(u.z, v.z)
    }

    #[test]
    fn matrix_rotation_90_90() {
        let (azimuth, elevation) = (90.0, 90.0);
        let mut camera = Camera::new(PhysicalSize::new(1920, 1080));
        camera.rotate2(azimuth, elevation);
        let rotation_matrix = camera.rotation_matrix();

        Camera::display_matrix("rotation 90 90", rotation_matrix);

        assert!(compare_vector3(-Vector3::unit_y(),
            (rotation_matrix * Vector4::<f32>::new(0.0, 0.0, 1.0, 1.0)).truncate()
            ));
    }

    fn camera_rotation(azimuth: f32, elevation: f32) -> Vector3<f32> {
        let mut camera = Camera::new(PhysicalSize::new(1920, 1080));
        camera.rotate2(azimuth, elevation);
        println!("camera.target = [{:.2}, {:.2}, {:.2}] camera.up = [{:.2}, {:.2}, {:.2}]",
            camera.target.x, camera.target.y, camera.target.z,
            camera.up.x, camera.up.y, camera.up.z,
        );
        let view = camera.get_view_matrix();
        let target: Vector3<f32> = (view * Vector4::unit_z()).truncate();
        target
    }

    #[test]
    fn screen_to_spheric_azimuth_0_elevation_90() { 

        let aspect = 1.0 / 1.0;
        let fovx = 90.0;
        let fovy = Camera::fovx_to_fovy(fovx, aspect);
        let camera_azimuth = 0.0;
        let camera_elevation = 90.0;
        
        let focal_length = 1.0 / (fovy * 0.5).to_radians().tan(); 
        println!("focal length : {:.2} fovx={:.2} fovy={:.2}", focal_length, fovx, fovy);
        let corner = Vector3::<f32>::new(1.0, 1.0, 1.0);
        println!("corner = {:?} corner.normalized = {:?}", corner, corner.normalize());
        let (corner_azimuth, corner_elevation) = cartesian_to_spheric(corner);

        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 90.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(-1.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 90.0+fovx*0.5));
        assert!(compare_f32(elevation, 90.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(1.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 90.0+fovx*0.5));
        assert!(compare_f32(elevation, 90.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, 1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 90.0+fovy*0.5));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, -1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 90.0-fovy*0.5));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(1.0, 1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, fovx));
        assert!(compare_f32(elevation, 90.0+fovy*0.5));
    }

    //#[test]
    fn screen_to_spheric_azimuth_90_elevation_90() { 
        let aspect = 2000.0 / 1000.0;
        let fovx = 90.0;
        let fovy = Camera::fovx_to_fovy(fovx, aspect);
        let camera_azimuth = 90.0;
        let camera_elevation = 90.0;
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 90.0));
        assert!(compare_f32(elevation, 90.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(-1.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 90.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(1.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 180.0));
        assert!(compare_f32(elevation, 90.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, 1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 90.0));
        assert!(compare_f32(elevation, 90.0+fovy));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, -1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 90.0));
        assert!(compare_f32(elevation, 90.0-fovy));
    }

    //#[test]
    fn screen_to_spheric_azimuth_0_elevation_0() { 
        let aspect = 2000.0 / 1000.0;
        let fovx = 90.0;
        let fovy = Camera::fovx_to_fovy(fovx, aspect);
        let camera_azimuth = 0.0;
        let camera_elevation = 0.1;
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 0.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(-1.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 90.0));
        assert!(compare_f32(elevation, 0.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(1.0, 0.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 270.0));
        assert!(compare_f32(elevation, 0.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, 1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 0.0));
        let (azimuth, elevation) = screen_to_spheric(cgmath::Point2::<f32>::new(0.0, -1.0), camera_azimuth, camera_elevation, fovx, aspect);
        assert!(compare_f32(azimuth, 0.0));
        assert!(compare_f32(elevation, 0.0));
    }

    #[test]
    fn vector_rotation() {
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 0.0, 0.0), Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 90.0, 0.0), Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 180.0, 0.0), Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 270.0, 0.0), Vector3::unit_y()));
        
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 0.0, 90.0), Vector3::unit_z()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 90.0, 90.0), Vector3::unit_x()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 180.0, 90.0), -Vector3::unit_z()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 270.0, 90.0), -Vector3::unit_x()));

        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 0.0, 180.0), -Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 90.0, 180.0), -Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 180.0, 180.0), -Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_y(), 270.0, 180.0), -Vector3::unit_y()));

        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 0.0, 0.0), Vector3::unit_z()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 90.0, 0.0), Vector3::unit_x()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 180.0, 0.0), -Vector3::unit_z()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 270.0, 0.0), -Vector3::unit_x()));
        
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 0.0, 90.0), -Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 90.0, 90.0), -Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 180.0, 90.0), -Vector3::unit_y()));
        assert!(compare_vector3(Camera::rotate_vector(Vector3::unit_z(), 270.0, 90.0), -Vector3::unit_y()));
    }
/*
    #[test]
    fn azimuth_0_elevation_90() {
        let target = camera_rotation(0.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 0.0, 1.0)));
    }

    #[test]
    fn azimuth_90_elevation_90() {
        let target = camera_rotation(90.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(-1.0, 0.0, 0.0)));
    }
    
    #[test]
    fn azimuth_180_elevation_90() {
        let target = camera_rotation(180.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn azimuth_270_elevation_90() {
        let target = camera_rotation(270.0, 90.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(1.0, 0.0, 0.0)));
    }

    #[test]
    fn azimuth_0_elevation_0() {
        let target = camera_rotation(0.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn azimuth_90_elevation_0() {
        let target = camera_rotation(90.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn azimuth_180_elevation_0() {
        let target = camera_rotation(180.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }

    #[test]
    fn azimuth_270_elevation_0() {
        let target = camera_rotation(270.0, 0.0); 
        println!("target = [{:.2}, {:.2}, {:.2}]", target.x, target.y, target.z);
        assert!(compare_vector3(target, Vector3::<f32>::new(0.0, 1.0, 0.0)));
    }
*/
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
