use crate::common::Pixel;

use super::{
    hsv_to_pixel::hsv_to_pixel, pixel_generator::PixelGenerator,
    pixel_generator_configuration::PixelGeneratorConfiguration,
};

struct HsvFixedValuePixelGenerator {
    pub value: f64,
    pub configuration: PixelGeneratorConfiguration,
}

impl PixelGenerator for HsvFixedValuePixelGenerator {
    fn configuration(&self) -> PixelGeneratorConfiguration {
        self.configuration
    }

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Pixel {
        hsv_to_pixel(angle_degrees, self.value, varying_dimension_value)
    }
}
