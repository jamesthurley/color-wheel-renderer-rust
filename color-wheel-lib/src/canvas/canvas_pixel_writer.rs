use crate::{pixel::Pixel, pixel_writer::PixelWriter};

use super::Canvas;

pub struct CanvasPixelWriter {
    pub canvas: Canvas,
}

impl CanvasPixelWriter {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            canvas: Canvas::new(width, height),
        }
    }
}

impl PixelWriter for CanvasPixelWriter {
    fn write_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        self.canvas.set_pixel(x, y, color);
    }
}
