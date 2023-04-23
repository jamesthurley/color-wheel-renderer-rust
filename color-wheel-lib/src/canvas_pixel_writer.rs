use crate::{
    canvas::Canvas,
    row_pixel_writer::{DefaultRowPixelWriter, RowPixelWriter},
};

pub trait CanvasPixelWriter {
    type RowPixelWriter<'canvas>: RowPixelWriter
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
        let chunk_size = self.canvas.width() as usize;
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
