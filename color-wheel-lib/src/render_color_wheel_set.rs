use crate::{
    canvas_pixel_writer::CanvasPixelWriter, canvas_pixel_writer_factory::CanvasPixelWriterFactory,
    color_wheel_definition::ColorWheelDefinition,
    offset_canvas_pixel_writer::OffsetCanvasPixelWriter, pixel_generators::PixelGenerator,
    render_color_wheel::RenderColorWheel,
};

pub trait RenderColorWheelSet {
    type Result: CanvasPixelWriter;

    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        color_wheels: &[ColorWheelDefinition<TPixelGenerator>],
        spacing: u32,
    ) -> Self::Result;
}

pub struct DefaultRenderColorWheelSet<TRenderColorWheel, TCanvasPixelWriterFactory>
where
    for<'canvas> TRenderColorWheel:
        RenderColorWheel<OffsetCanvasPixelWriter<'canvas, TCanvasPixelWriterFactory::Result>>,
    for<'canvas> TCanvasPixelWriterFactory: CanvasPixelWriterFactory,
{
    pub render_color_wheel: TRenderColorWheel,
    pub pixel_writer_factory: TCanvasPixelWriterFactory,
}

impl<TRenderColorWheel, TCanvasPixelWriterFactory> RenderColorWheelSet
    for DefaultRenderColorWheelSet<TRenderColorWheel, TCanvasPixelWriterFactory>
where
    for<'canvas> TRenderColorWheel:
        RenderColorWheel<OffsetCanvasPixelWriter<'canvas, TCanvasPixelWriterFactory::Result>>,
    for<'canvas> TCanvasPixelWriterFactory: CanvasPixelWriterFactory,
{
    type Result = TCanvasPixelWriterFactory::Result;

    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        color_wheels: &[ColorWheelDefinition<TPixelGenerator>],
        spacing: u32,
    ) -> TCanvasPixelWriterFactory::Result {
        if color_wheels.is_empty() {
            panic!("No color wheels to render.");
        }

        let color_wheels_count: u32 = color_wheels
            .len()
            .try_into()
            .expect("Too many color wheels.");

        // Color wheels are laid out horizontally, so we take the max size for the height
        // and the sum of the sizes with padding added for the width.
        let overall_height = color_wheels.iter().map(|v| v.image_size).max().unwrap();
        let overall_width = color_wheels.iter().map(|v| v.image_size).sum::<u32>()
            + (spacing * (color_wheels_count - 1));

        let mut pixel_writer = self
            .pixel_writer_factory
            .create(overall_width, overall_height);
        let mut offset_x = 0;

        for color_wheel in color_wheels {
            let mut offset_pixel_writer = OffsetCanvasPixelWriter {
                canvas_pixel_writer: &mut pixel_writer,
                offset_x,
                offset_y: 0,
            };

            self.render_color_wheel
                .execute(color_wheel, &mut offset_pixel_writer);
            offset_x += color_wheel.image_size + spacing;
        }

        pixel_writer
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        pixel_generators::{MockPixelGenerator, PixelGenerator},
        row_pixel_writer::MockRowPixelWriter,
    };

    use super::*;

    #[test]
    fn it_should_render_all_color_wheels() {
        let color_wheel_renderer: Rc<MockRenderColorWheel> = Default::default();
        let pixel_writer_factory: Rc<MockPixelWriterFactory> = Default::default();

        let render_color_wheel_set = DefaultRenderColorWheelSet {
            render_color_wheel: Rc::clone(&color_wheel_renderer),
            pixel_writer_factory: Rc::clone(&pixel_writer_factory),
        };

        let color_wheels = vec![
            ColorWheelDefinition {
                image_size: 100,
                margin_size: 10,
                angle_buckets: 36,
                distance_buckets: 5,
                pixel_generators: vec![MockPixelGenerator::new()],
            },
            ColorWheelDefinition {
                image_size: 200,
                margin_size: 20,
                angle_buckets: 36,
                distance_buckets: 5,
                pixel_generators: vec![MockPixelGenerator::new()],
            },
        ];

        render_color_wheel_set.execute(&color_wheels, 10);

        assert_eq!(pixel_writer_factory.calls.take(), vec![(310, 200)]);

        assert_eq!(
            color_wheel_renderer.calls.take(),
            vec![
                RenderColorWheelCall {
                    wheel_size: 100,
                    offset_x: 0,
                    offset_y: 0
                },
                RenderColorWheelCall {
                    wheel_size: 200,
                    offset_x: 110,
                    offset_y: 0
                },
            ]
        );
    }

    struct MockCanvasPixelWriter {}

    impl CanvasPixelWriter for MockCanvasPixelWriter {
        type RowPixelWriter<'canvas> = MockRowPixelWriter;

        fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>> {
            unreachable!();
        }
    }

    #[derive(Default)]
    struct MockPixelWriterFactory {
        pub calls: RefCell<Vec<(u32, u32)>>,
    }
    impl CanvasPixelWriterFactory for Rc<MockPixelWriterFactory> {
        type Result = MockCanvasPixelWriter;

        fn create(&self, width: u32, height: u32) -> Self::Result {
            self.calls.borrow_mut().push((width, height));
            MockCanvasPixelWriter {}
        }
    }

    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    struct RenderColorWheelCall {
        pub wheel_size: u32,
        pub offset_x: u32,
        pub offset_y: u32,
    }

    #[derive(Default)]
    struct MockRenderColorWheel {
        pub calls: RefCell<Vec<RenderColorWheelCall>>,
    }
    impl<'canvas, TPixelWriter: CanvasPixelWriter>
        RenderColorWheel<OffsetCanvasPixelWriter<'canvas, TPixelWriter>>
        for Rc<MockRenderColorWheel>
    {
        fn execute<TPixelGenerator: PixelGenerator>(
            &self,
            definition: &ColorWheelDefinition<TPixelGenerator>,
            pixel_writer: &mut OffsetCanvasPixelWriter<'canvas, TPixelWriter>,
        ) {
            self.calls.borrow_mut().push(RenderColorWheelCall {
                wheel_size: definition.image_size,
                offset_x: pixel_writer.offset_x,
                offset_y: pixel_writer.offset_y,
            });
        }
    }
}
