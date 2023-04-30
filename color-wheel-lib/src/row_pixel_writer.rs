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

        let index = BYTES_PER_PIXEL * x as usize;
        self.data[index..index + BYTES_PER_PIXEL].copy_from_slice(&pixel.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_write_to_correct_buffer_locations() {
        let mut pixel_writer = DefaultRowPixelWriter {
            row_index: 2,
            data: &mut [0, 0, 0, 0, 0, 0, 0, 0],
        };

        pixel_writer.write_pixel(0, 2, Pixel::rgb(1, 2, 3));
        pixel_writer.write_pixel(1, 2, Pixel::rgb(4, 5, 6));

        assert_eq!(pixel_writer.data, &[1, 2, 3, 255, 4, 5, 6, 255]);
    }

    #[test]
    #[should_panic]
    fn it_should_verify_row() {
        let mut pixel_writer = DefaultRowPixelWriter {
            row_index: 2,
            data: &mut [0, 0, 0, 0, 0, 0, 0, 0],
        };

        pixel_writer.write_pixel(0, 1, Pixel::rgb(1, 2, 3));
    }
}
