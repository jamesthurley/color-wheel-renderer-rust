use crate::common::Pixel;

#[cfg_attr(test, mockall::automock)]
pub trait PixelWriter {
    fn write_pixel(&mut self, x: usize, y: usize, color: Pixel);
}

pub trait PixelWriterFactory {
    type Result: PixelWriter;

    fn create(&self, width: usize, height: usize) -> Self::Result;
}
