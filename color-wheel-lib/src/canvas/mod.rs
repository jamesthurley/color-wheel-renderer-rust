use crate::pixel::BYTES_PER_PIXEL;

use super::pixel::Pixel;
use super::pixel::TRANSPARENT;

pub mod canvas_pixel_writer;
pub mod canvas_pixel_writer_factory;
mod get_pixel;
mod get_ppm;
mod set_pixel;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![TRANSPARENT; width * height * BYTES_PER_PIXEL],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.data.chunks(BYTES_PER_PIXEL).map(|chunk| chunk.into())
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_initialize_to_transparent() {
        let result = canvas(2, 3);

        assert_eq!(result.width(), 2);
        assert_eq!(result.height(), 3);
        assert_eq!(result.data.len(), 6 * BYTES_PER_PIXEL);

        let transparent = Pixel::transparent();

        for y in 0..result.height() {
            for x in 0..result.width() {
                assert_eq!(result.get_pixel(x, y), transparent);
            }
        }
    }
}
