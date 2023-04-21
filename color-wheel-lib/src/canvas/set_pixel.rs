use super::*;

impl Canvas {
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if x >= self.width || y >= self.height {
            panic!("Requested pixel location is out of bounds: {x},{y}");
        }

        let pixel_start = (x as usize + y as usize * self.width as usize) * BYTES_PER_PIXEL;

        self.data[pixel_start..pixel_start + BYTES_PER_PIXEL].copy_from_slice(&pixel.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_update_specified_pixel() {
        let mut target = canvas(2, 3);
        let transparent = Pixel::transparent();

        assert!(target.iter_pixels().all(|v| { v == transparent }));

        target.set_pixel(1, 1, Pixel::rgb(1, 2, 3));

        assert_eq!(target.get_pixel(1, 1), Pixel::rgb(1, 2, 3));
    }

    #[test]
    #[should_panic]
    fn when_x_out_of_bounds_it_should_panic() {
        let mut target = canvas(2, 3);
        target.set_pixel(2, 0, Pixel::rgb(1, 2, 3));
    }

    #[test]
    #[should_panic]
    fn when_y_out_of_bounds_it_should_panic() {
        let mut target = canvas(2, 3);
        target.set_pixel(0, 3, Pixel::rgb(1, 2, 3));
    }
}
