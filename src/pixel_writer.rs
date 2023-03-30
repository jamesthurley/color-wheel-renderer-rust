use crate::common::Pixel;

#[cfg_attr(test, mockall::automock)]
pub trait PixelWriter {
    fn write_pixel(&mut self, x: usize, y: usize, color: Pixel);
}

pub trait PixelWriterFactory {
    type Result: PixelWriter;

    fn create(&self, width: usize, height: usize) -> Self::Result;
}

pub struct OffsetPixelWriter<'w, TPixelWriter>
where
    TPixelWriter: PixelWriter,
{
    pub pixel_writer: &'w mut TPixelWriter,
    pub offset_x: usize,
    pub offset_y: usize,
}

impl<'w, TPixelWriter> PixelWriter for OffsetPixelWriter<'w, TPixelWriter>
where
    TPixelWriter: PixelWriter,
{
    fn write_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        self.pixel_writer
            .write_pixel(x + self.offset_x, y + self.offset_y, color);
    }
}
