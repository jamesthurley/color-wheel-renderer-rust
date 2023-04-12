use crate::{
    color_wheel_definition::ColorWheelDefinition,
    offset_pixel_writer::OffsetPixelWriter,
    pixel_generators::PixelGenerator,
    pixel_writer::{PixelWriter, PixelWriterFactory},
    render_color_wheel::RenderColorWheel,
};

pub trait RenderColorWheelSet<TPixelWriter: PixelWriter> {
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        color_wheels: &[ColorWheelDefinition<TPixelGenerator>],
        spacing: usize,
    ) -> TPixelWriter;
}

pub struct DefaultRenderColorWheelSet<TRenderColorWheel, TPixelWriterFactory>
where
    for<'pw> TRenderColorWheel:
        RenderColorWheel<OffsetPixelWriter<'pw, TPixelWriterFactory::Result>>,
    TPixelWriterFactory: PixelWriterFactory,
{
    pub render_color_wheel: TRenderColorWheel,
    pub pixel_writer_factory: TPixelWriterFactory,
}

impl<TRenderColorWheel, TPixelWriterFactory> RenderColorWheelSet<TPixelWriterFactory::Result>
    for DefaultRenderColorWheelSet<TRenderColorWheel, TPixelWriterFactory>
where
    for<'pw> TRenderColorWheel:
        RenderColorWheel<OffsetPixelWriter<'pw, TPixelWriterFactory::Result>>,
    TPixelWriterFactory: PixelWriterFactory,
{
    fn execute<TPixelGenerator: PixelGenerator>(
        &self,
        color_wheels: &[ColorWheelDefinition<TPixelGenerator>],
        spacing: usize,
    ) -> TPixelWriterFactory::Result {
        if color_wheels.is_empty() {
            panic!("No color wheels to render.");
        }

        // Color wheels are laid out horizontally, so we take the max size for the height
        // and the sum of the sizes with padding added for the width.
        let overall_height = color_wheels.iter().map(|v| v.image_size).max().unwrap();
        let overall_width = color_wheels.iter().map(|v| v.image_size).sum::<usize>()
            + (spacing * (color_wheels.len() - 1));

        let mut pixel_writer = self
            .pixel_writer_factory
            .create(overall_width, overall_height);
        let mut offset_x = 0;

        for color_wheel in color_wheels {
            let mut offset_pixel_writer = OffsetPixelWriter {
                pixel_writer: &mut pixel_writer,
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
        pixel_writer::{MockPixelWriter, PixelWriter},
    };

    use super::*;

    #[test]
    fn it_should_render_all_color_wheels() {
        let color_wheel_renderer: Rc<MockRenderColorWheel> = Default::default();
        let pixel_writer_factory: Rc<MockPixelWriterFactory> = Default::default();

        let color_wheel_set_renderer = DefaultRenderColorWheelSet {
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

        color_wheel_set_renderer.execute(&color_wheels, 10);

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

    #[derive(Default)]
    struct MockPixelWriterFactory {
        pub calls: RefCell<Vec<(usize, usize)>>,
    }
    impl PixelWriterFactory for Rc<MockPixelWriterFactory> {
        type Result = MockPixelWriter;

        fn create(&self, width: usize, height: usize) -> Self::Result {
            self.calls.borrow_mut().push((width, height));
            MockPixelWriter::new()
        }
    }

    #[derive(Eq, PartialEq, Copy, Clone, Debug)]
    struct RenderColorWheelCall {
        pub wheel_size: usize,
        pub offset_x: usize,
        pub offset_y: usize,
    }

    #[derive(Default)]
    struct MockRenderColorWheel {
        pub calls: RefCell<Vec<RenderColorWheelCall>>,
    }
    impl<'pw, TPixelWriter: PixelWriter> RenderColorWheel<OffsetPixelWriter<'pw, TPixelWriter>>
        for Rc<MockRenderColorWheel>
    {
        fn execute<TPixelGenerator: PixelGenerator>(
            &self,
            definition: &ColorWheelDefinition<TPixelGenerator>,
            pixel_writer: &mut OffsetPixelWriter<'pw, TPixelWriter>,
        ) {
            self.calls.borrow_mut().push(RenderColorWheelCall {
                wheel_size: definition.image_size,
                offset_x: pixel_writer.offset_x,
                offset_y: pixel_writer.offset_y,
            });
        }
    }
}
