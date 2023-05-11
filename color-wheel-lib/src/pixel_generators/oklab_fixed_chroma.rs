use oklab::{oklab_to_srgb, Oklab};

use crate::pixel::Pixel;

use super::{
    pixel_generator::PixelGenerator, pixel_generator_configuration::PixelGeneratorConfiguration,
};

pub struct OklabFixedChromaPixelGenerator {
    pub chroma: f64,
    pub configuration: PixelGeneratorConfiguration,
}

impl PixelGenerator for OklabFixedChromaPixelGenerator {
    fn configuration(&self) -> PixelGeneratorConfiguration {
        self.configuration
    }

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Pixel {
        let angle_rad = angle_degrees.to_radians();

        let c = self.chroma;

        let l = varying_dimension_value as f32;
        let a = (c * angle_rad.cos()) as f32;
        let b = (c * angle_rad.sin()) as f32;

        let srgb = oklab_to_srgb(Oklab { l, a, b });

        Pixel::rgb(srgb.r, srgb.g, srgb.b)
    }
}
