use crate::pixel::Pixel;

use super::{
    hsv_to_pixel::hsv_to_pixel, pixel_generator::PixelGenerator,
    pixel_generator_configuration::PixelGeneratorConfiguration,
};

pub struct HsvFixedSaturationPixelGenerator {
    pub saturation: f64,
    pub configuration: PixelGeneratorConfiguration,
}

impl PixelGenerator for HsvFixedSaturationPixelGenerator {
    fn configuration(&self) -> PixelGeneratorConfiguration {
        self.configuration
    }

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Pixel {
        hsv_to_pixel(angle_degrees, self.saturation, varying_dimension_value)
    }
}
