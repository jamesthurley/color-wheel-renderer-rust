use crate::{pixel::Pixel, pixel_writer::PixelWriter};

use super::Canvas;

pub struct CanvasPixelWriter {
    pub canvas: Canvas,
}

impl CanvasPixelWriter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Canvas::new(width, height),
        }
    }
}

impl PixelWriter for CanvasPixelWriter {
    fn write_pixel(&mut self, x: u32, y: u32, color: Pixel) {
        self.canvas.set_pixel(x, y, color);
    }
}
