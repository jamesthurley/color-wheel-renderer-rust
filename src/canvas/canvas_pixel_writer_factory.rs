use crate::pixel_writer::PixelWriterFactory;

use super::canvas_pixel_writer::CanvasPixelWriter;

pub struct CanvasPixelWriterFactory {}

impl PixelWriterFactory for CanvasPixelWriterFactory {
    type Result = CanvasPixelWriter;

    fn create(&self, width: usize, height: usize) -> Self::Result {
        CanvasPixelWriter::new(width, height)
    }
}
