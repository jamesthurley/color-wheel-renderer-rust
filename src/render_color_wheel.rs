use std::cmp::min;

use crate::{
    color_wheel_definition::ColorWheelDefinition,
    pixel_generators::pixel_generator::PixelGenerator,
    pixel_writer::PixelWriter,
    render_pixel::{RenderPixel, RenderPixelData},
};

// We're putting the `PixelWriter` as a generic parameter on the `ColorWheelRenderer` trait
// so that we can more easily mock it in `ColorWheelSetRenderer`.
pub trait RenderColorWheel<TPixelWriter: PixelWriter> {
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    );
}

pub struct DefaultRenderColorWheel<TRenderPixel>
where
    TRenderPixel: RenderPixel,
{
    render_pixel: TRenderPixel,
}

impl<TRenderPixel: RenderPixel, TPixelWriter: PixelWriter> RenderColorWheel<TPixelWriter>
    for DefaultRenderColorWheel<TRenderPixel>
{
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    ) {
        if definition.pixel_generators.is_empty() {
            return;
        }

        let image_width = definition.image_size;
        let image_height = definition.image_size;

        let wheel_diameter = min(image_width, image_height) - (definition.margin_size * 2);

        let center_x = image_width / 2;
        let center_y = image_height / 2;

        let all_generators_size = wheel_diameter as f64 / 2.;

        let generator_size = all_generators_size / definition.pixel_generators.len() as f64;
        if generator_size < 1. {
            panic!("Image is too small.");
        }

        let data = RenderPixelData {
            center_x,
            center_y,
            all_generators_size,
            generator_size,
        };

        for image_y in 0..image_height {
            for image_x in 0..image_width {
                self.render_pixel
                    .execute(image_x, image_y, &data, definition, pixel_writer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use float_cmp::assert_approx_eq;

    use crate::{
        pixel_generators::pixel_generator::MockPixelGenerator, pixel_writer::MockPixelWriter,
    };

    use super::*;

    #[test]
    fn when_no_pixel_generators_it_should_return() {
        let mut pixel_writer = MockPixelWriter::new();

        let render_pixel = Rc::new(MockRenderPixel {
            calls: RefCell::new(vec![]),
        });

        let definition = ColorWheelDefinition::<MockPixelGenerator> {
            image_size: 15,
            margin_size: 2,
            angle_buckets: 4,
            distance_buckets: 5,
            pixel_generators: vec![],
        };

        let renderer = DefaultRenderColorWheel {
            render_pixel: render_pixel.clone(),
        };

        renderer.execute(&definition, &mut pixel_writer);

        let calls = render_pixel.calls.borrow();
        assert_eq!(calls.len(), 0);
    }

    #[test]
    fn it_should_render_each_pixel() {
        let mut pixel_writer = MockPixelWriter::new();

        let render_pixel = Rc::new(MockRenderPixel {
            calls: RefCell::new(vec![]),
        });

        let definition = ColorWheelDefinition {
            image_size: 15,
            margin_size: 2,
            angle_buckets: 4,
            distance_buckets: 5,
            pixel_generators: vec![MockPixelGenerator::new(), MockPixelGenerator::new()],
        };

        let renderer = DefaultRenderColorWheel {
            render_pixel: render_pixel.clone(),
        };

        renderer.execute(&definition, &mut pixel_writer);

        let calls = render_pixel.calls.borrow();
        assert_eq!(calls.len(), 225);

        let call = &calls[17];

        assert_eq!(call.image_x, 2);
        assert_eq!(call.image_y, 1);
        assert_eq!(call.data.center_x, 7);
        assert_eq!(call.data.center_y, 7);
        assert_approx_eq!(f64, call.data.all_generators_size, 5.5);
        assert_approx_eq!(f64, call.data.generator_size, 2.75);
    }

    struct MockRenderPixelCall {
        image_x: usize,
        image_y: usize,
        data: RenderPixelData,
    }
    struct MockRenderPixel {
        calls: RefCell<Vec<MockRenderPixelCall>>,
    }
    impl RenderPixel for Rc<MockRenderPixel> {
        fn execute<TPixelGenerator: PixelGenerator, TPixelWriter: PixelWriter>(
            &self,
            image_x: usize,
            image_y: usize,
            data: &RenderPixelData,
            _definition: &ColorWheelDefinition<TPixelGenerator>,
            _pixel_writer: &mut TPixelWriter,
        ) {
            self.calls.borrow_mut().push(MockRenderPixelCall {
                image_x,
                image_y,
                data: *data,
            });
        }
    }
}
