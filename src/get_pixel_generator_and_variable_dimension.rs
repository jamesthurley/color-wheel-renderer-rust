use crate::{
    color_wheel_definition::ColorWheelDefinition, pixel_generators::pixel_generator::PixelGenerator,
};

pub struct PixelGeneratorAndVariableDimension<'a, TPixelGenerator: PixelGenerator> {
    pub pixel_generator: &'a TPixelGenerator,
    pub variable_dimension: f64,
}

pub trait GetPixelGeneratorAndVariableDimension {
    fn execute<'a, TPixelGenerator: PixelGenerator>(
        &self,
        generator_size: f64,
        definition: &'a ColorWheelDefinition<TPixelGenerator>,
        distance_from_center: f64,
    ) -> Option<PixelGeneratorAndVariableDimension<'a, TPixelGenerator>>;
}

struct DefaultGetPixelGeneratorAndVariableDimension {}
impl GetPixelGeneratorAndVariableDimension for DefaultGetPixelGeneratorAndVariableDimension {
    fn execute<'a, TPixelGenerator: PixelGenerator>(
        &self,
        generator_size: f64,
        definition: &'a ColorWheelDefinition<TPixelGenerator>,
        distance_from_center: f64,
    ) -> Option<PixelGeneratorAndVariableDimension<'a, TPixelGenerator>> {
        let mut variable_dimension = 1.;
        let mut generator_inner_distance = 0.;
        let mut generator_outer_distance = generator_size;
        let mut pixel_generator = None;

        for current_pixel_generator in definition.pixel_generators.iter() {
            if distance_from_center < generator_outer_distance {
                variable_dimension =
                    (distance_from_center - generator_inner_distance) / generator_size;
                pixel_generator = Some(current_pixel_generator);
                break;
            }

            generator_inner_distance = generator_outer_distance;
            generator_outer_distance += generator_size;
        }

        pixel_generator.map(|pixel_generator| PixelGeneratorAndVariableDimension {
            pixel_generator,
            variable_dimension,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use float_cmp::assert_approx_eq;

    use crate::{bucket::BucketDirection, common::Pixel};

    use super::*;

    struct Setup {
        pub target: DefaultGetPixelGeneratorAndVariableDimension,
        pub generator_size: f64,
        pub definition: ColorWheelDefinition<Rc<MockPixelGenerator>>,
    }

    fn setup() -> Setup {
        Setup {
            target: DefaultGetPixelGeneratorAndVariableDimension {},
            generator_size: 40.,
            definition: ColorWheelDefinition {
                image_size: 140,
                margin_size: 10,
                angle_buckets: 10,
                distance_buckets: 10,
                pixel_generators: vec![
                    Rc::new(MockPixelGenerator { id: 1 }),
                    Rc::new(MockPixelGenerator { id: 2 }),
                    Rc::new(MockPixelGenerator { id: 3 }),
                ],
            },
        }
    }

    #[test]
    fn outside_generators() {
        let setup = setup();

        let result = setup
            .target
            .execute(setup.generator_size, &setup.definition, 121.);

        assert!(result.is_none());
    }

    #[test]
    fn within_inner_generator() {
        let setup = setup();

        let result = setup
            .target
            .execute(setup.generator_size, &setup.definition, 38.);

        let result = result.unwrap();
        assert_eq!(result.pixel_generator.id, 1);
        assert_approx_eq!(f64, result.variable_dimension, 38. / 40.);
    }

    #[test]
    fn within_middle_generator() {
        let setup = setup();

        let result = setup
            .target
            .execute(setup.generator_size, &setup.definition, 75.);

        let result = result.unwrap();
        assert_eq!(result.pixel_generator.id, 2);
        assert_approx_eq!(f64, result.variable_dimension, 35. / 40.);
    }

    #[test]
    fn within_outer_generator() {
        let setup = setup();

        let result = setup
            .target
            .execute(setup.generator_size, &setup.definition, 119.);

        let result = result.unwrap();
        assert_eq!(result.pixel_generator.id, 3);
        assert_approx_eq!(f64, result.variable_dimension, 39. / 40.);
    }

    struct MockPixelGenerator {
        id: usize,
    }
    impl PixelGenerator for Rc<MockPixelGenerator> {
        fn is_angle_inverted(&self) -> bool {
            unimplemented!()
        }

        fn is_varying_dimension_inverted(&self) -> bool {
            unimplemented!()
        }

        fn angle_bucket_direction(&self) -> BucketDirection {
            unimplemented!()
        }

        fn varying_dimension_bucket_direction(&self) -> BucketDirection {
            unimplemented!()
        }

        fn get_pixel(&self, _angle_degrees: f64, _varying_dimension_value: f64) -> Option<Pixel> {
            unimplemented!()
        }
    }
}
