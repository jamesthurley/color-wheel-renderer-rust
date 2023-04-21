use crate::{
    color_wheel_definition::ColorWheelDefinition, get_angle_degrees::get_angle_degrees,
    get_pixel::GetPixel,
    get_pixel_generator_and_variable_dimension::GetPixelGeneratorAndVariableDimension,
    pixel_generators::PixelGenerator, pixel_writer::PixelWriter,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderPixelData {
    pub center_x: u32,
    pub center_y: u32,
    pub all_generators_size: f64,
    pub generator_size: f64,
}

pub trait RenderPixel {
    fn execute<TPixelGenerator: PixelGenerator, TPixelWriter: PixelWriter>(
        &self,
        image_x: u32,
        image_y: u32,
        data: &RenderPixelData,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    );
}

pub struct DefaultRenderPixel<TGetPixelGeneratorAndVariableDimension, TGetPixel>
where
    TGetPixelGeneratorAndVariableDimension: GetPixelGeneratorAndVariableDimension,
    TGetPixel: GetPixel,
{
    pub get_pixel_generator_and_variable_dimension: TGetPixelGeneratorAndVariableDimension,
    pub get_pixel: TGetPixel,
}

impl<TGetPixelGeneratorAndVariableDimension, TGetPixel> RenderPixel
    for DefaultRenderPixel<TGetPixelGeneratorAndVariableDimension, TGetPixel>
where
    TGetPixelGeneratorAndVariableDimension: GetPixelGeneratorAndVariableDimension,
    TGetPixel: GetPixel,
{
    fn execute<TPixelGenerator: PixelGenerator, TPixelWriter: PixelWriter>(
        &self,
        image_x: u32,
        image_y: u32,
        data: &RenderPixelData,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    ) {
        let relative_x = image_x as f64 - data.center_x as f64;
        let relative_y = image_y as f64 - data.center_y as f64;
        let distance_from_center = (relative_x.powi(2) + relative_y.powi(2)).sqrt();

        if distance_from_center > data.all_generators_size {
            return;
        }

        let pixel_generator_result = self.get_pixel_generator_and_variable_dimension.execute(
            data.generator_size,
            definition,
            distance_from_center,
        );

        if pixel_generator_result.is_none() {
            return;
        }

        let angle_degrees = get_angle_degrees(0., 0., relative_x, relative_y);

        let pixel_generator_result = pixel_generator_result.unwrap();
        let pixel_generator = pixel_generator_result.pixel_generator;
        let variable_dimension = pixel_generator_result.variable_dimension;

        let pixel = self.get_pixel.execute(
            pixel_generator,
            angle_degrees,
            variable_dimension,
            definition.angle_buckets,
            definition.distance_buckets,
        );

        pixel_writer.write_pixel(image_x, image_y, pixel);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use float_cmp::assert_approx_eq;
    use mockall::predicate::*;

    use crate::{
        get_pixel_generator_and_variable_dimension::PixelGeneratorAndVariableDimension,
        pixel::Pixel, pixel_generators::MockPixelGenerator, pixel_writer::MockPixelWriter,
    };

    use super::*;

    struct SetupData {
        pub pixel_writer: MockPixelWriter,
        pub get_pixel_generator_and_variable_dimension:
            Rc<MockGetPixelGeneratorAndVariableDimension>,
        pub get_pixel: Rc<MockGetPixel>,
        pub target:
            DefaultRenderPixel<Rc<MockGetPixelGeneratorAndVariableDimension>, Rc<MockGetPixel>>,
        pub render_pixel_data: RenderPixelData,
        pub color_wheel_definition: ColorWheelDefinition<MockPixelGenerator>,
    }

    fn setup(generator_index: isize, variable_dimension: f64, pixel: Pixel) -> SetupData {
        let pixel_writer = MockPixelWriter::new();

        let get_pixel_generator_and_variable_dimension =
            Rc::new(MockGetPixelGeneratorAndVariableDimension {
                result_index: generator_index,
                variable_dimension,
                calls: RefCell::new(vec![]),
            });

        let get_pixel = Rc::new(MockGetPixel {
            result: pixel,
            calls: RefCell::new(vec![]),
        });

        let target =
            DefaultRenderPixel::<Rc<MockGetPixelGeneratorAndVariableDimension>, Rc<MockGetPixel>> {
                get_pixel_generator_and_variable_dimension:
                    get_pixel_generator_and_variable_dimension.clone(),
                get_pixel: get_pixel.clone(),
            };

        let render_pixel_data = RenderPixelData {
            center_x: 55,
            center_y: 55,
            all_generators_size: 50.,
            generator_size: 25.,
        };

        let color_wheel_definition = ColorWheelDefinition::<MockPixelGenerator> {
            image_size: 110,
            margin_size: 5,
            angle_buckets: 10,
            distance_buckets: 10,
            pixel_generators: vec![MockPixelGenerator::new(), MockPixelGenerator::new()],
        };

        SetupData {
            pixel_writer,
            get_pixel_generator_and_variable_dimension,
            get_pixel,
            target,
            render_pixel_data,
            color_wheel_definition,
        }
    }

    #[test]
    fn when_outside_of_wheel_it_should_return() {
        let mut test = setup(-1, 0., Default::default());

        test.target.execute(
            1,
            1,
            &test.render_pixel_data,
            &test.color_wheel_definition,
            &mut test.pixel_writer,
        );

        assert_eq!(
            test.get_pixel_generator_and_variable_dimension
                .calls
                .borrow()
                .len(),
            0
        );

        assert_eq!(test.get_pixel.calls.borrow().len(), 0);
    }

    #[test]
    fn when_no_pixel_generator_returned_it_should_return() {
        let mut test = setup(-1, 0., Default::default());

        test.target.execute(
            56,
            54,
            &test.render_pixel_data,
            &test.color_wheel_definition,
            &mut test.pixel_writer,
        );

        assert_eq!(
            test.get_pixel_generator_and_variable_dimension
                .calls
                .borrow()
                .len(),
            1
        );

        let call = &test
            .get_pixel_generator_and_variable_dimension
            .calls
            .borrow()[0];

        assert_eq!(call.generator_size, test.render_pixel_data.generator_size);
        assert_approx_eq!(f64, call.distance_from_center, std::f64::consts::SQRT_2);

        assert_eq!(test.get_pixel.calls.borrow().len(), 0);
    }

    #[test]
    fn when_pixel_generator_returns_pixel_it_should_write_pixel() {
        let pixel = Pixel::rgb(1, 2, 3);
        let mut test = setup(0, 123., pixel);

        test.pixel_writer
            .expect_write_pixel()
            .with(eq(56), eq(54), eq(pixel))
            .once()
            .return_const(());

        test.target.execute(
            56,
            54,
            &test.render_pixel_data,
            &test.color_wheel_definition,
            &mut test.pixel_writer,
        );

        assert_eq!(test.get_pixel.calls.borrow().len(), 1);

        let call = &test.get_pixel.calls.borrow()[0];
        assert_approx_eq!(f64, call.angle_degrees, 45.);
        assert_approx_eq!(f64, call.variable_dimension, 123.);
        assert_eq!(
            call.angle_buckets,
            test.color_wheel_definition.angle_buckets
        );
        assert_eq!(
            call.distance_buckets,
            test.color_wheel_definition.distance_buckets
        );
    }

    struct MockGetPixelGeneratorAndVariableDimensionCall {
        generator_size: f64,
        distance_from_center: f64,
    }

    struct MockGetPixelGeneratorAndVariableDimension {
        result_index: isize,
        variable_dimension: f64,
        calls: RefCell<Vec<MockGetPixelGeneratorAndVariableDimensionCall>>,
    }

    impl GetPixelGeneratorAndVariableDimension for Rc<MockGetPixelGeneratorAndVariableDimension> {
        fn execute<'a, TPixelGenerator: PixelGenerator>(
            &self,
            generator_size: f64,
            definition: &'a ColorWheelDefinition<TPixelGenerator>,
            distance_from_center: f64,
        ) -> Option<PixelGeneratorAndVariableDimension<'a, TPixelGenerator>> {
            self.calls
                .borrow_mut()
                .push(MockGetPixelGeneratorAndVariableDimensionCall {
                    generator_size,
                    distance_from_center,
                });

            if self.result_index < 0 {
                return None;
            }

            Some(PixelGeneratorAndVariableDimension {
                pixel_generator: &definition.pixel_generators[self.result_index as usize],
                variable_dimension: self.variable_dimension,
            })
        }
    }

    struct MockGetPixelCall {
        angle_degrees: f64,
        variable_dimension: f64,
        angle_buckets: u32,
        distance_buckets: u32,
    }
    struct MockGetPixel {
        result: Pixel,
        calls: RefCell<Vec<MockGetPixelCall>>,
    }
    impl GetPixel for Rc<MockGetPixel> {
        fn execute<TPixelGenerator: PixelGenerator>(
            &self,
            _pixel_generator: &TPixelGenerator,
            angle_degrees: f64,
            variable_dimension: f64,
            angle_buckets: u32,
            distance_buckets: u32,
        ) -> Pixel {
            self.calls.borrow_mut().push(MockGetPixelCall {
                angle_degrees,
                variable_dimension,
                angle_buckets,
                distance_buckets,
            });

            self.result
        }
    }
}
