use crate::{pixel::Pixel, row_pixel_writer::RowPixelWriter};

pub struct OffsetRowPixelWriter<TRowPixelWriter>
where
    TRowPixelWriter: RowPixelWriter,
{
    pub row_pixel_writer: TRowPixelWriter,
    pub offset_x: u32,
    pub offset_y: u32,
}

impl<TRowPixelWriter> RowPixelWriter for OffsetRowPixelWriter<TRowPixelWriter>
where
    TRowPixelWriter: RowPixelWriter,
{
    fn write_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.row_pixel_writer
            .write_pixel(x + self.offset_x, y + self.offset_y, pixel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        todo!();
    }
}
