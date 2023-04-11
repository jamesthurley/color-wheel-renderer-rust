use super::*;

impl Canvas {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Pixel) {
        if x >= self.width || y >= self.height {
            panic!("Requested pixel location is out of bounds: {x},{y}");
        }

        self.data[x + y * self.width] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_update_specified_pixel() {
        let mut target = canvas(2, 3);
        let black = Pixel::new(0, 0, 0);

        assert!(target.data().iter().all(|v| { v == &black }));

        target.set_pixel(1, 1, Pixel::new(1, 2, 3));

        assert_eq!(target.get_pixel(1, 1), &Pixel::new(1, 2, 3));
    }

    #[test]
    #[should_panic]
    fn when_x_out_of_bounds_it_should_panic() {
        let mut target = canvas(2, 3);
        target.set_pixel(2, 0, Pixel::new(1, 2, 3));
    }

    #[test]
    #[should_panic]
    fn when_y_out_of_bounds_it_should_panic() {
        let mut target = canvas(2, 3);
        target.set_pixel(0, 3, Pixel::new(1, 2, 3));
    }
}
