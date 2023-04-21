use crate::pixel::Pixel;

#[cfg_attr(test, mockall::automock)]
pub trait PixelWriter {
    fn write_pixel(&mut self, x: u32, y: u32, color: Pixel);
}

pub trait PixelWriterFactory {
    type Result: PixelWriter;

    fn create(&self, width: u32, height: u32) -> Self::Result;
}
