use color_wheel_lib::{bucket::BucketDirection, pixel_generators::PixelGeneratorConfiguration};

use crate::cli::Cli;

pub fn create_pixel_generator_configuration(cli: &Cli) -> PixelGeneratorConfiguration {
    let radial_bucket_direction: BucketDirection;
    if !cli.reverse_radial_colors {
        if !cli.reverse_radial_bucketing {
            radial_bucket_direction = BucketDirection::Up;
        } else {
            radial_bucket_direction = BucketDirection::Down;
        }
    } else if !cli.reverse_radial_bucketing {
        radial_bucket_direction = BucketDirection::Down;
    } else {
        radial_bucket_direction = BucketDirection::Up;
    }

    PixelGeneratorConfiguration {
        is_angle_inverted: false,
        is_varying_dimension_inverted: cli.reverse_radial_colors,
        angle_bucket_direction: BucketDirection::Down,
        varying_dimension_bucket_direction: radial_bucket_direction,
    }
}
