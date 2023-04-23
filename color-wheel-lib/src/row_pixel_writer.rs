use crate::pixel::{Pixel, BYTES_PER_PIXEL};

#[cfg_attr(test, mockall::automock)]
pub trait RowPixelWriter {
    fn write_pixel(&mut self, x: u32, y: u32, pixel: Pixel);
}

pub struct DefaultRowPixelWriter<'canvas> {
    pub row_index: u32,
    pub data: &'canvas mut [u8],
}

impl<'canvas> RowPixelWriter for DefaultRowPixelWriter<'canvas> {
    fn write_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if y != self.row_index {
            panic!(
                "RowPixelWriter asked to write to row {} when row {} was expected.",
                y, self.row_index
            );
        }

        let index = x as usize;
        self.data[index..index + BYTES_PER_PIXEL].copy_from_slice(&pixel.data);
    }
}
