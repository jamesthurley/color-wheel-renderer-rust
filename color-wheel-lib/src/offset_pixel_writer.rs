use crate::{pixel::Pixel, pixel_writer::PixelWriter};

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

#[cfg(test)]
mod tests {
    use crate::pixel_writer::MockPixelWriter;

    use super::*;

    #[test]
    fn offset_pixel_writer_should_offset_x_and_y() {
        let mut pixel_writer = MockPixelWriter::new();

        let pixel = Pixel::rgb(1, 2, 3);

        pixel_writer
            .expect_write_pixel()
            .with(
                mockall::predicate::eq(11),
                mockall::predicate::eq(22),
                mockall::predicate::eq(pixel),
            )
            .once()
            .return_const(());

        let mut offset_pixel_writer = OffsetPixelWriter {
            pixel_writer: &mut pixel_writer,
            offset_x: 10,
            offset_y: 20,
        };

        offset_pixel_writer.write_pixel(1, 2, pixel);
    }
}
