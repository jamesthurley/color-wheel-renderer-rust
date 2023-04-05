use crate::{bucket::bucket, common::Pixel, pixel_generators::pixel_generator::PixelGenerator};

pub trait GetPixel {
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        pixel_generator: &TPixelGenerator,
        angle_degrees: f64,
        variable_dimension: f64,
        angle_buckets: usize,
        distance_buckets: usize,
    ) -> Option<Pixel>;
}

pub struct DefaultGetPixel {}
impl GetPixel for DefaultGetPixel {
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        pixel_generator: &TPixelGenerator,
        mut angle_degrees: f64,
        mut variable_dimension: f64,
        angle_buckets: usize,
        distance_buckets: usize,
    ) -> Option<Pixel> {
        angle_degrees = bucket(
            angle_degrees,
            360.,
            angle_buckets,
            pixel_generator.angle_bucket_direction(),
        );

        variable_dimension = bucket(
            variable_dimension,
            1.,
            distance_buckets,
            pixel_generator.varying_dimension_bucket_direction(),
        );

        if pixel_generator.is_angle_inverted() {
            angle_degrees = 360. - angle_degrees;
        }

        if pixel_generator.is_varying_dimension_inverted() {
            variable_dimension = 1. - variable_dimension;
        }

        println!("angle_degrees: {}", angle_degrees);
        println!("variable_dimension: {}", variable_dimension);
        pixel_generator.get_pixel(angle_degrees, variable_dimension)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bucket::BucketDirection, pixel_generators::pixel_generator::MockPixelGenerator};
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
            .expect_varying_dimension_bucket_direction()
            .return_const(varying_dimension_bucket_direction);

        pixel_generator
            .expect_angle_bucket_direction()
            .return_const(angle_bucket_direction);

        pixel_generator
            .expect_is_varying_dimension_inverted()
            .return_const(is_varying_dimension_inverted);

        pixel_generator
            .expect_is_angle_inverted()
            .return_const(is_angle_inverted);

        let expected_pixel = Pixel::new(1, 2, 3);
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
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

        assert_eq!(pixel, Some(expected_pixel));
    }
}
