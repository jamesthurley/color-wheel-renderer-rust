use crate::common::Pixel;

use super::{
    hsl_to_pixel::hsl_to_pixel, pixel_generator::PixelGenerator,
    pixel_generator_configuration::PixelGeneratorConfiguration,
};

struct HslFixedSaturationPixelGenerator {
    pub saturation: f64,
    pub configuration: PixelGeneratorConfiguration,
}

impl PixelGenerator for HslFixedSaturationPixelGenerator {
    fn configuration(&self) -> PixelGeneratorConfiguration {
        self.configuration
    }

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Pixel {
        hsl_to_pixel(angle_degrees, self.saturation, varying_dimension_value)
    }
}
