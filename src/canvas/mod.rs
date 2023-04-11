use super::common::Pixel;

mod get_pixel;
mod get_ppm;
mod set_pixel;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![Pixel::new(0, 0, 0); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &[Pixel] {
        &self.data
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_initialize_to_black() {
        let result = canvas(2, 3);

        assert_eq!(result.width(), 2);
        assert_eq!(result.height(), 3);
        assert_eq!(result.data().len(), 6);
        for pixel in result.data() {
            assert_eq!(pixel.red, 0);
            assert_eq!(pixel.green, 0);
            assert_eq!(pixel.blue, 0);
        }
    }
}
