use crate::pixel::Pixel;

use super::pixel_generator_configuration::PixelGeneratorConfiguration;

#[cfg_attr(test, mockall::automock)]
pub trait PixelGenerator {
    fn configuration(&self) -> PixelGeneratorConfiguration;

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Pixel;
}
