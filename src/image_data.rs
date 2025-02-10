use image::{
    ImageReader,
    GenericImageView,
    RgbaImage,
};

pub struct ImageData {
    //image: DynamicImage,
    diffuse_rgba: RgbaImage,
    dimensions: (u32, u32),
}

impl ImageData {

    pub fn new(filename: &str) -> Result<Self, &'static str> { 
        //let diffuse_bytes = include_bytes!(filename);
        //let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();

        println!("Opening image \"{filename}\"");
        let image = match ImageReader::open(filename).unwrap().decode() {
            Err(why) => {panic!("{}", why)}
            Ok(value) => value
        };

        let diffuse_rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        Ok(Self{
            diffuse_rgba,
            dimensions,
        })    
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn rgba(&self) -> &RgbaImage {
        &self.diffuse_rgba
    }
}
