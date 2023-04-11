use super::*;

impl Canvas {
    pub fn get_pixel(&self, x: usize, y: usize) -> &Pixel {
        if x >= self.width || y >= self.height {
            panic!("Requested pixel is out of bounds: {x},{y}");
        }

        &self.data[x + y * self.width]
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
                target.set_pixel(x, y, Pixel::new(x as u8, y as u8, 1));
            }
        }

        assert_eq!(target.get_pixel(0, 0), &Pixel::new(0, 0, 1));
        assert_eq!(target.get_pixel(1, 2), &Pixel::new(1, 2, 1));
        assert_eq!(target.get_pixel(1, 1), &Pixel::new(1, 1, 1));
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
