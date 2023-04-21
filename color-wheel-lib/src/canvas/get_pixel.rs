use crate::pixel::BYTES_PER_PIXEL;

use super::*;

impl Canvas {
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        if x >= self.width || y >= self.height {
            panic!("Requested pixel is out of bounds: {x},{y}");
        }

        let pixel_start = (x as usize + y as usize * self.width as usize) * BYTES_PER_PIXEL;

        self.data[pixel_start..pixel_start + BYTES_PER_PIXEL].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_should_return_pixel_at_location() {
        let mut target = canvas(2, 3);

        for x in 0..2 {
            for y in 0..3 {
                target.set_pixel(x, y, Pixel::rgba(x as u8, y as u8, 100, 200));
            }
        }

        assert_eq!(target.get_pixel(0, 0), Pixel::rgba(0, 0, 100, 200));
        assert_eq!(target.get_pixel(1, 2), Pixel::rgba(1, 2, 100, 200));
        assert_eq!(target.get_pixel(1, 1), Pixel::rgba(1, 1, 100, 200));
    }

    #[test]
    #[should_panic]
    pub fn when_x_out_of_bounds_it_should_panic() {
        let target = canvas(2, 3);
        target.get_pixel(2, 0);
    }

    #[test]
    #[should_panic]
    pub fn when_y_out_of_bounds_it_should_panic() {
        let target = canvas(2, 3);
        target.get_pixel(0, 3);
    }
}
