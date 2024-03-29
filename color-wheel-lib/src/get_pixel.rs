use crate::{bucket::bucket, pixel::Pixel, pixel_generators::PixelGenerator};

pub trait GetPixel: Sync {
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        pixel_generator: &TPixelGenerator,
        angle_degrees: f64,
        variable_dimension: f64,
        angle_buckets: u32,
        distance_buckets: u32,
    ) -> Pixel;
}

pub struct DefaultGetPixel {}
impl GetPixel for DefaultGetPixel {
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        pixel_generator: &TPixelGenerator,
        mut angle_degrees: f64,
        mut variable_dimension: f64,
        angle_buckets: u32,
        distance_buckets: u32,
    ) -> Pixel {
        let configuration = pixel_generator.configuration();

        angle_degrees = bucket(
            angle_degrees,
            360.,
            angle_buckets,
            configuration.angle_bucket_direction,
        );

        variable_dimension = bucket(
            variable_dimension,
            1.,
            distance_buckets,
            configuration.varying_dimension_bucket_direction,
        );

        if configuration.is_angle_inverted {
            angle_degrees = 360. - angle_degrees;
        }

        if configuration.is_varying_dimension_inverted {
            variable_dimension = 1. - variable_dimension;
        }

        pixel_generator.get_pixel(angle_degrees, variable_dimension)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bucket::BucketDirection,
        pixel_generators::{MockPixelGenerator, PixelGeneratorConfiguration},
    };
    use mockall::predicate::*;

    use super::*;

    fn setup(
        pixel_generator: &mut MockPixelGenerator,
        angle_bucket_direction: BucketDirection,
        varying_dimension_bucket_direction: BucketDirection,
        is_angle_inverted: bool,
        is_varying_dimension_inverted: bool,
        expected_angle_degrees: f64,
        expected_variable_dimension: f64,
    ) -> Pixel {
        pixel_generator
            .expect_configuration()
            .return_const(PixelGeneratorConfiguration {
                varying_dimension_bucket_direction,
                angle_bucket_direction,
                is_varying_dimension_inverted,
                is_angle_inverted,
            });

        let expected_pixel = Pixel::rgb(1, 2, 3);
        pixel_generator
            .expect_get_pixel()
            .with(
                float::is_close(expected_angle_degrees),
                float::is_close(expected_variable_dimension),
            )
            .once()
            .return_const(expected_pixel);

        expected_pixel
    }

    #[test]
    fn it_should_get_pixel() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Up,
            BucketDirection::Up,
            false,
            false,
            135.,
            0.2,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 0, 0);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_bucket_angle_up() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Up,
            BucketDirection::Up,
            false,
            false,
            180.,
            0.2,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 4, 0);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_bucket_angle_down() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Down,
            BucketDirection::Up,
            false,
            false,
            90.,
            0.2,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 4, 0);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_bucket_variable_dimension_up() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Up,
            BucketDirection::Up,
            false,
            false,
            135.,
            0.25,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 0, 4);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_bucket_varying_dimension_down() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Up,
            BucketDirection::Down,
            false,
            false,
            135.,
            0.0,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 0, 4);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_invert_angle() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Up,
            BucketDirection::Up,
            true,
            false,
            225.,
            0.2,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 0, 0);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_invert_varying_dimension() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Up,
            BucketDirection::Up,
            false,
            true,
            135.,
            0.8,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 0, 0);

        assert_eq!(pixel, expected_pixel);
    }

    #[test]
    fn it_should_bucket_before_inverting() {
        let mut pixel_generator = MockPixelGenerator::new();

        let expected_pixel = setup(
            &mut pixel_generator,
            BucketDirection::Down,
            BucketDirection::Up,
            true,
            true,
            270.,
            0.75,
        );

        let get_pixel = DefaultGetPixel {};
        let pixel = get_pixel.execute(&pixel_generator, 135., 0.2, 4, 4);

        assert_eq!(pixel, expected_pixel);
    }
}
