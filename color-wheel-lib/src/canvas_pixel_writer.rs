use crate::{
    canvas::Canvas,
    pixel::BYTES_PER_PIXEL,
    row_pixel_writer::{DefaultRowPixelWriter, RowPixelWriter},
};

pub trait CanvasPixelWriter {
    type RowPixelWriter<'canvas>: RowPixelWriter + Send
    where
        Self: 'canvas;

    fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>>;
}

pub struct DefaultCanvasPixelWriter {
    pub canvas: Canvas,
}

impl DefaultCanvasPixelWriter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Canvas::new(width, height),
        }
    }
}

impl CanvasPixelWriter for DefaultCanvasPixelWriter {
    type RowPixelWriter<'canvas> = DefaultRowPixelWriter<'canvas>;

    fn rows_mut(&mut self) -> Vec<Self::RowPixelWriter<'_>> {
        let chunk_size = BYTES_PER_PIXEL * self.canvas.width() as usize;
        let row_count = self.canvas.height() as usize;
        let chunks = self.canvas.data_mut().chunks_mut(chunk_size);

        let mut result: Vec<Self::RowPixelWriter<'_>> = Vec::with_capacity(row_count);

        for (row_index, chunk) in chunks.into_iter().enumerate() {
            result.push(DefaultRowPixelWriter {
                row_index: row_index.try_into().expect("Row index was too large."),
                data: chunk,
            });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::pixel::Pixel;

    use super::*;

    #[test]
    fn it_should_write_to_correct_buffer_locations() {
        let mut pixel_writer = DefaultCanvasPixelWriter::new(3, 3);

        let mut rows = pixel_writer.rows_mut();

        assert_eq!(rows.len(), 3);

        rows[0].write_pixel(0, 0, Pixel::rgb(1, 2, 3));
        rows[2].write_pixel(2, 2, Pixel::rgb(4, 5, 6));

        assert_eq!(pixel_writer.canvas.get_pixel(0, 0), Pixel::rgb(1, 2, 3));
        assert_eq!(pixel_writer.canvas.get_pixel(2, 2), Pixel::rgb(4, 5, 6));
    }
}
