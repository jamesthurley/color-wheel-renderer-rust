#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_normalized(red: f64, green: f64, blue: f64) -> Self {
        fn denormalize(value: f64) -> u8 {
            (value * 255.).round() as u8
        }

        Self::new(denormalize(red), denormalize(green), denormalize(blue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_normalized() {
        assert_eq!(Pixel::from_normalized(1., 0., 0.), Pixel::new(255, 0, 0));

        assert_eq!(Pixel::from_normalized(0., 1., 0.), Pixel::new(0, 255, 0));

        assert_eq!(Pixel::from_normalized(0., 0., 1.), Pixel::new(0, 0, 255));

        assert_eq!(
            Pixel::from_normalized(0.5, 0.5, 0.5),
            Pixel::new(128, 128, 128)
        );

        assert_eq!(
            Pixel::from_normalized(0.25, 0.25, 0.25),
            Pixel::new(64, 64, 64)
        );

        assert_eq!(
            Pixel::from_normalized(0.75, 0.75, 0.75),
            Pixel::new(191, 191, 191)
        );
    }
}
