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
    use crate::row_pixel_writer::MockRowPixelWriter;

    use super::*;

    #[test]
    fn offset_pixel_writer_should_offset_x_and_y() {
        let mut pixel_writer = MockRowPixelWriter::new();

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

        let mut offset_pixel_writer = OffsetRowPixelWriter {
            row_pixel_writer: pixel_writer,
            offset_x: 10,
            offset_y: 20,
        };

        offset_pixel_writer.write_pixel(1, 2, pixel);
    }
}
