use crate::{
    canvas_pixel_writer::CanvasPixelWriter, offset_row_pixel_writer::OffsetRowPixelWriter,
};

pub struct OffsetCanvasPixelWriter<'canvas, TCanvasPixelWriter>
where
    TCanvasPixelWriter: CanvasPixelWriter,
{
    pub canvas_pixel_writer: &'canvas mut TCanvasPixelWriter,
    pub offset_x: u32,
    pub offset_y: u32,
}

impl<'canvas, TCanvasPixelWriter> CanvasPixelWriter
    for OffsetCanvasPixelWriter<'canvas, TCanvasPixelWriter>
where
    TCanvasPixelWriter: CanvasPixelWriter + 'canvas,
{
    type RowPixelWriter<'inner> = OffsetRowPixelWriter<TCanvasPixelWriter::RowPixelWriter<'inner>>
        where 'canvas : 'inner;

    fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>> {
        self.canvas_pixel_writer
            .rows_mut()
            .into_iter()
            .skip(self.offset_y as usize)
            .map(|row| OffsetRowPixelWriter {
                offset_x: self.offset_x,
                offset_y: self.offset_y,
                row_pixel_writer: row,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::row_pixel_writer::MockPixelWriter;

    use super::*;

    #[test]
    fn offset_pixel_writer_should_offset_x_and_y() {
        let mut pixel_writer = MockCanvasPixelWriter::new();

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

        let mut offset_pixel_writer = OffsetCanvasPixelWriter {
            canvas_pixel_writer: &mut pixel_writer,
            offset_x: 10,
            offset_y: 20,
        };

        offset_pixel_writer.write_pixel(1, 2, pixel);
    }
}
