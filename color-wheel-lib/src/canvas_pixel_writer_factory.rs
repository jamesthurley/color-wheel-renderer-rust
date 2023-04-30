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

#[cfg(test)]
mod tests {
    use crate::pixel::BYTES_PER_PIXEL;

    use super::*;

    #[test]
    fn it_should_return_canvas_with_correctly_sized_buffer() {
        let factory = DefaultCanvasPixelWriterFactory {};
        let canvas = factory.create(3, 4);

        assert_eq!(canvas.canvas.data().len(), 3 * 4 * BYTES_PER_PIXEL);
    }
}
