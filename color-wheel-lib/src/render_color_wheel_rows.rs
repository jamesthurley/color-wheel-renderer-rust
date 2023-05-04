use crate::{
    canvas_pixel_writer::CanvasPixelWriter,
    color_wheel_definition::ColorWheelDefinition,
    pixel_generators::PixelGenerator,
    render_pixel::{RenderPixel, RenderPixelData},
};
use rayon::prelude::*;

pub trait RenderColorWheelRows {
    fn execute<TPixelGenerator: PixelGenerator, TCanvasPixelWriter>(
        &self,
        image_width: u32,
        image_height: u32,
        data: &RenderPixelData,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        canvas_pixel_writer: &mut TCanvasPixelWriter,
    ) where
        TCanvasPixelWriter: CanvasPixelWriter;
}

pub struct DefaultRenderColorWheelRows<TRenderPixel>
where
    TRenderPixel: RenderPixel,
{
    pub render_pixel: TRenderPixel,
}

impl<TRenderPixel> RenderColorWheelRows for DefaultRenderColorWheelRows<TRenderPixel>
where
    TRenderPixel: RenderPixel,
{
    fn execute<TPixelGenerator: PixelGenerator, TCanvasPixelWriter>(
        &self,
        image_width: u32,
        image_height: u32,
        data: &RenderPixelData,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        canvas_pixel_writer: &mut TCanvasPixelWriter,
    ) where
        TCanvasPixelWriter: CanvasPixelWriter,
    {
        let mut rows = canvas_pixel_writer.rows_mut();

        if rows.len() < image_height as usize {
            panic!(
                "Expected color wheel image height was {} but only {} canvas rows were returned.",
                image_height,
                rows.len()
            );
        }

        // Use Rayon to parallelize the loop over the rows.
        rows.par_iter_mut().enumerate().for_each(|(image_y, row)| {
            for image_x in 0..image_width {
                self.render_pixel
                    .execute(image_x, image_y as u32, data, definition, row);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{
        pixel::Pixel, pixel_generators::MockPixelGenerator, row_pixel_writer::RowPixelWriter,
    };

    use super::*;

    #[test]
    fn it_should_render_each_pixel() {
        let rows = (0..2)
            .map(|_| Arc::new(Mutex::new(MockRowPixelWriter::new())))
            .collect::<Vec<Arc<Mutex<MockRowPixelWriter>>>>();

        let mut canvas_pixel_writer = MockCanvasPixelWriter { rows: rows.clone() };

        let render_pixel = Arc::new(MockRenderPixel {
            calls: Mutex::new(vec![]),
        });

        let definition = random_color_wheel_definition();
        let data = random_render_pixel_data();

        let renderer = DefaultRenderColorWheelRows {
            render_pixel: render_pixel.clone(),
        };

        renderer.execute(2, 2, &data, &definition, &mut canvas_pixel_writer);

        let calls = render_pixel.calls.lock().unwrap();
        assert_eq!(calls.len(), 4);

        assert!(calls.iter().all(|c| c.data == data));
        assert_eq!(
            calls
                .iter()
                .map(|c| (c.image_x, c.image_y))
                .collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (0, 1), (1, 1),]
        );

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].lock().unwrap().calls, vec![(0, 0), (1, 0)]);
        assert_eq!(rows[1].lock().unwrap().calls, vec![(0, 1), (1, 1)]);
    }

    #[test]
    #[should_panic]
    fn when_not_enough_rows_it_should_panic() {
        let rows = (0..2)
            .map(|_| Arc::new(Mutex::new(MockRowPixelWriter::new())))
            .collect::<Vec<Arc<Mutex<MockRowPixelWriter>>>>();

        let mut canvas_pixel_writer = MockCanvasPixelWriter { rows };

        let render_pixel = Arc::new(MockRenderPixel {
            calls: Mutex::new(vec![]),
        });

        let definition = random_color_wheel_definition();
        let data = random_render_pixel_data();

        let renderer = DefaultRenderColorWheelRows { render_pixel };

        renderer.execute(2, 3, &data, &definition, &mut canvas_pixel_writer);
    }

    fn random_render_pixel_data() -> RenderPixelData {
        RenderPixelData {
            center_x: 7,
            center_y: 7,
            all_generators_size: 5.5,
            generator_size: 2.75,
        }
    }

    fn random_color_wheel_definition() -> ColorWheelDefinition<MockPixelGenerator> {
        ColorWheelDefinition {
            image_size: 15,
            margin_size: 2,
            angle_buckets: 4,
            distance_buckets: 5,
            pixel_generators: vec![MockPixelGenerator::new(), MockPixelGenerator::new()],
        }
    }

    struct MockRowPixelWriter {
        pub calls: Vec<(u32, u32)>,
    }
    impl MockRowPixelWriter {
        pub fn new() -> Self {
            MockRowPixelWriter { calls: vec![] }
        }
    }
    impl RowPixelWriter for Arc<Mutex<MockRowPixelWriter>> {
        fn write_pixel(&mut self, x: u32, y: u32, _pixel: crate::pixel::Pixel) {
            self.lock().unwrap().calls.push((x, y));
        }
    }

    struct MockCanvasPixelWriter {
        pub rows: Vec<Arc<Mutex<MockRowPixelWriter>>>,
    }
    impl CanvasPixelWriter for MockCanvasPixelWriter {
        type RowPixelWriter<'canvas> = Arc<Mutex<MockRowPixelWriter>>;

        fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>> {
            self.rows.clone()
        }
    }

    struct MockRenderPixelCall {
        image_x: u32,
        image_y: u32,
        data: RenderPixelData,
    }
    struct MockRenderPixel {
        calls: Mutex<Vec<MockRenderPixelCall>>,
    }
    impl RenderPixel for Arc<MockRenderPixel> {
        fn execute<TPixelGenerator: PixelGenerator, TRowPixelWriter: RowPixelWriter>(
            &self,
            image_x: u32,
            image_y: u32,
            data: &RenderPixelData,
            _definition: &ColorWheelDefinition<TPixelGenerator>,
            pixel_writer: &mut TRowPixelWriter,
        ) {
            pixel_writer.write_pixel(image_x, image_y, Pixel::transparent());
            self.calls.lock().unwrap().push(MockRenderPixelCall {
                image_x,
                image_y,
                data: *data,
            });
        }
    }
}
