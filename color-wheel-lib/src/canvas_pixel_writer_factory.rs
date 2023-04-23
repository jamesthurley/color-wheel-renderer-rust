use crate::canvas_pixel_writer::{CanvasPixelWriter, DefaultCanvasPixelWriter};

pub trait CanvasPixelWriterFactory {
    type Result: CanvasPixelWriter;

    fn create(&self, width: u32, height: u32) -> Self::Result;
}

pub struct DefaultCanvasPixelWriterFactory {}

impl CanvasPixelWriterFactory for DefaultCanvasPixelWriterFactory {
    type Result = DefaultCanvasPixelWriter;

    fn create(&self, width: u32, height: u32) -> Self::Result {
        DefaultCanvasPixelWriter::new(width, height)
    }
}
