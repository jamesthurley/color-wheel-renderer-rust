use crate::bucket::BucketDirection;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PixelGeneratorConfiguration {
    pub is_angle_inverted: bool,
    pub is_varying_dimension_inverted: bool,
    pub angle_bucket_direction: BucketDirection,
    pub varying_dimension_bucket_direction: BucketDirection,
}
