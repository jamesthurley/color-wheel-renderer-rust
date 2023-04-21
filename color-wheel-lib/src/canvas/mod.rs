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
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![TRANSPARENT; width as usize * height as usize * BYTES_PER_PIXEL],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.data.chunks(BYTES_PER_PIXEL).map(|chunk| chunk.into())
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn eject_data(self) -> Vec<u8> {
        self.data
    }
}

pub fn canvas(width: u32, height: u32) -> Canvas {
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

    #[test]
    fn it_should_iterate_pixels() {
        let mut canvas = canvas(2, 2);

        let pixels = vec![
            Pixel::rgba(0, 1, 2, 3),
            Pixel::rgba(4, 5, 6, 7),
            Pixel::rgba(7, 6, 5, 4),
            Pixel::rgba(3, 2, 1, 0),
        ];

        canvas.set_pixel(0, 0, pixels[0]);
        canvas.set_pixel(1, 0, pixels[1]);
        canvas.set_pixel(0, 1, pixels[2]);
        canvas.set_pixel(1, 1, pixels[3]);

        let result: Vec<Pixel> = canvas.iter_pixels().collect();

        assert_eq!(result, pixels);
    }
}
