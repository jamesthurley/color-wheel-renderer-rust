use crate::pixel::Pixel;

use super::{
    hsl_to_pixel::hsl_to_pixel, pixel_generator::PixelGenerator,
    pixel_generator_configuration::PixelGeneratorConfiguration,
};

pub struct HslFixedLightnessPixelGenerator {
    pub lightness: f64,
    pub configuration: PixelGeneratorConfiguration,
}

impl PixelGenerator for HslFixedLightnessPixelGenerator {
    fn configuration(&self) -> PixelGeneratorConfiguration {
        self.configuration
    }

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Pixel {
        hsl_to_pixel(angle_degrees, varying_dimension_value, self.lightness)
    }
}
