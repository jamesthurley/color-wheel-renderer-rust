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

    use crate::{
        canvas_pixel_writer::DefaultCanvasPixelWriter,
        pixel::Pixel,
        row_pixel_writer::{MockRowPixelWriter, RowPixelWriter},
    };

    use super::*;

    #[test]
    fn offset_pixel_writer_should_return_offset_row_pixel_writers() {
        let mut canvas_pixel_writer = MockCanvasPixelWriter {};

        let mut offset_pixel_writer = OffsetCanvasPixelWriter {
            canvas_pixel_writer: &mut canvas_pixel_writer,
            offset_x: 3,
            offset_y: 4,
        };

        let rows = offset_pixel_writer.rows_mut();

        assert_eq!(rows.len(), 6);
        assert!(rows
            .iter()
            .all(|row| row.offset_x == 3 && row.offset_y == 4));
    }

    struct MockCanvasPixelWriter {}

    impl CanvasPixelWriter for MockCanvasPixelWriter {
        type RowPixelWriter<'canvas> = MockRowPixelWriter;

        fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>> {
            (0..10).map(|_| MockRowPixelWriter::new()).collect()
        }
    }

    #[test]
    fn it_should_write_to_correct_buffer_locations() {
        let mut canvas_pixel_writer = DefaultCanvasPixelWriter::new(4, 4);

        let mut offset_pixel_writer = OffsetCanvasPixelWriter {
            canvas_pixel_writer: &mut canvas_pixel_writer,
            offset_x: 1,
            offset_y: 2,
        };

        let mut rows = offset_pixel_writer.rows_mut();

        assert_eq!(rows.len(), 2);

        rows[0].write_pixel(0, 0, Pixel::rgb(1, 2, 3));
        rows[1].write_pixel(2, 1, Pixel::rgb(4, 5, 6));

        assert_eq!(
            canvas_pixel_writer.canvas.get_pixel(1, 2),
            Pixel::rgb(1, 2, 3)
        );
        assert_eq!(
            canvas_pixel_writer.canvas.get_pixel(3, 3),
            Pixel::rgb(4, 5, 6)
        );
    }
}
