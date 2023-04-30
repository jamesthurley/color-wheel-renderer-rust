use std::cmp::min;

use crate::{
    canvas_pixel_writer::CanvasPixelWriter, color_wheel_definition::ColorWheelDefinition,
    pixel_generators::PixelGenerator, render_color_wheel_rows::RenderColorWheelRows,
    render_pixel::RenderPixelData,
};

// We're putting the `PixelWriter` as a generic parameter on the `RenderColorWheel` trait
// so that we can more easily mock it in `RenderColorWheelSet`.
pub trait RenderColorWheel<TCanvasPixelWriter>
where
    TCanvasPixelWriter: CanvasPixelWriter,
{
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        canvas_pixel_writer: &mut TCanvasPixelWriter,
    );
}

pub struct DefaultRenderColorWheel<TRenderColorWheelRows: RenderColorWheelRows>
where
    TRenderColorWheelRows: RenderColorWheelRows,
{
    pub render_color_wheel_rows: TRenderColorWheelRows,
}

impl<TRenderColorWheelRows, TCanvasPixelWriter> RenderColorWheel<TCanvasPixelWriter>
    for DefaultRenderColorWheel<TRenderColorWheelRows>
where
    TRenderColorWheelRows: RenderColorWheelRows,
    TCanvasPixelWriter: CanvasPixelWriter,
{
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        canvas_pixel_writer: &mut TCanvasPixelWriter,
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

        self.render_color_wheel_rows.execute(
            image_width,
            image_height,
            &data,
            definition,
            canvas_pixel_writer,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use float_cmp::assert_approx_eq;

    use crate::{pixel_generators::MockPixelGenerator, row_pixel_writer::MockRowPixelWriter};

    use super::*;

    #[test]
    fn when_no_pixel_generators_it_should_return() {
        let mut canvas_pixel_writer = MockCanvasPixelWriter {};

        let render_color_wheel_rows = Rc::new(MockRenderColorWheelRows {
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
            render_color_wheel_rows: render_color_wheel_rows.clone(),
        };

        renderer.execute(&definition, &mut canvas_pixel_writer);

        let calls = render_color_wheel_rows.calls.borrow();
        assert_eq!(calls.len(), 0);
    }

    #[test]
    fn it_should_render_each_pixel() {
        let mut canvas_pixel_writer = MockCanvasPixelWriter {};

        let render_color_wheel_rows = Rc::new(MockRenderColorWheelRows {
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
            render_color_wheel_rows: render_color_wheel_rows.clone(),
        };

        renderer.execute(&definition, &mut canvas_pixel_writer);

        let calls = render_color_wheel_rows.calls.borrow();
        assert_eq!(calls.len(), 1);

        let call = &calls[0];

        assert_eq!(call.image_width, 15);
        assert_eq!(call.image_height, 15);
        assert_eq!(call.data.center_x, 7);
        assert_eq!(call.data.center_y, 7);
        assert_approx_eq!(f64, call.data.all_generators_size, 5.5);
        assert_approx_eq!(f64, call.data.generator_size, 2.75);
    }

    struct MockCanvasPixelWriter {}

    impl CanvasPixelWriter for MockCanvasPixelWriter {
        type RowPixelWriter<'canvas> = MockRowPixelWriter;

        fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>> {
            unreachable!()
        }
    }

    struct MockRenderColorWheelRowsCall {
        image_width: u32,
        image_height: u32,
        data: RenderPixelData,
    }

    struct MockRenderColorWheelRows {
        calls: RefCell<Vec<MockRenderColorWheelRowsCall>>,
    }

    impl RenderColorWheelRows for Rc<MockRenderColorWheelRows> {
        fn execute<TPixelGenerator: PixelGenerator, TCanvasPixelWriter>(
            &self,
            image_width: u32,
            image_height: u32,
            data: &RenderPixelData,
            _definition: &ColorWheelDefinition<TPixelGenerator>,
            _canvas_pixel_writer: &mut TCanvasPixelWriter,
        ) where
            TCanvasPixelWriter: CanvasPixelWriter,
        {
            self.calls.borrow_mut().push(MockRenderColorWheelRowsCall {
                image_width,
                image_height,
                data: *data,
            });
        }
    }
}
