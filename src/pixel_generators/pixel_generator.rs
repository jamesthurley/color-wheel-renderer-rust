use crate::{bucket::BucketDirection, common::Pixel};

#[cfg_attr(test, mockall::automock)]
pub trait PixelGenerator {
    fn is_angle_inverted(&self) -> bool;
    fn is_varying_dimension_inverted(&self) -> bool;
    fn angle_bucket_direction(&self) -> BucketDirection;
    fn varying_dimension_bucket_direction(&self) -> BucketDirection;

    fn get_pixel(&self, angle_degrees: f64, varying_dimension_value: f64) -> Option<Pixel>;
}
