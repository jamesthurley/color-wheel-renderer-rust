pub const TRANSPARENT: u8 = 0;
pub const OPAQUE: u8 = 255;

pub const BYTES_PER_PIXEL: usize = 4;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Pixel {
    pub data: [u8; BYTES_PER_PIXEL],
}

impl Pixel {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            data: [red, green, blue, OPAQUE],
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            data: [red, green, blue, alpha],
        }
    }

    pub fn transparent() -> Self {
        Self {
            data: [0, 0, 0, TRANSPARENT],
        }
    }

    pub fn from_normalized(red: f64, green: f64, blue: f64) -> Self {
        fn denormalize(value: f64) -> u8 {
            (value * 255.).round() as u8
        }

        Self::rgb(denormalize(red), denormalize(green), denormalize(blue))
    }

    pub fn red(&self) -> u8 {
        self.data[0]
    }

    pub fn green(&self) -> u8 {
        self.data[1]
    }

    pub fn blue(&self) -> u8 {
        self.data[2]
    }

    pub fn alpha(&self) -> u8 {
        self.data[3]
    }
}

impl From<Pixel> for [u8; BYTES_PER_PIXEL] {
    fn from(pixel: Pixel) -> Self {
        pixel.data
    }
}

impl From<[u8; BYTES_PER_PIXEL]> for Pixel {
    fn from(data: [u8; BYTES_PER_PIXEL]) -> Self {
        Self { data }
    }
}

impl From<&[u8]> for Pixel {
    fn from(data: &[u8]) -> Self {
        let pixel_data = <&[u8; BYTES_PER_PIXEL]>::try_from(data)
            .expect("Pixel data slice was not the expected size.");
        Self { data: *pixel_data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_normalized() {
        assert_eq!(
            Pixel::from_normalized(1., 0., 0.),
            Pixel::rgba(255, 0, 0, OPAQUE)
        );

        assert_eq!(
            Pixel::from_normalized(0., 1., 0.),
            Pixel::rgba(0, 255, 0, OPAQUE)
        );

        assert_eq!(
            Pixel::from_normalized(0., 0., 1.),
            Pixel::rgba(0, 0, 255, OPAQUE)
        );

        assert_eq!(
            Pixel::from_normalized(0.5, 0.5, 0.5),
            Pixel::rgba(128, 128, 128, OPAQUE)
        );

        assert_eq!(
            Pixel::from_normalized(0.25, 0.25, 0.25),
            Pixel::rgba(64, 64, 64, OPAQUE)
        );

        assert_eq!(
            Pixel::from_normalized(0.75, 0.75, 0.75),
            Pixel::rgba(191, 191, 191, OPAQUE)
        );
    }
}
