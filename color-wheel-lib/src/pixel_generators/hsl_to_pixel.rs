use crate::pixel::Pixel;

use super::intermediate_cxm_to_pixel::intermediate_cxm_to_pixel;

pub fn hsl_to_pixel(mut hue: f64, mut saturation: f64, mut lightness: f64) -> Pixel {
    hue = hue.clamp(0., 360.);
    saturation = saturation.clamp(0., 1.);
    lightness = lightness.clamp(0., 1.);

    // https://www.rapidtables.com/convert/color/hsl-to-rgb.html
    let c = (1. - ((2. * lightness) - 1.).abs()) * saturation;
    let x = c * (1. - ((hue / 60.) % 2. - 1.).abs());
    let m = lightness - (c / 2.);

    intermediate_cxm_to_pixel(hue, c, x, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_from_hsl_to_pixel() {
        assert_eq!(
            hsl_to_pixel(0., 1., 0.5),
            Pixel::from_normalized(1., 0., 0.)
        );

        assert_eq!(
            hsl_to_pixel(60., 1., 0.5),
            Pixel::from_normalized(1., 1., 0.)
        );

        assert_eq!(
            hsl_to_pixel(120., 1., 0.5),
            Pixel::from_normalized(0., 1., 0.)
        );

        assert_eq!(
            hsl_to_pixel(180., 1., 0.5),
            Pixel::from_normalized(0., 1., 1.)
        );

        assert_eq!(
            hsl_to_pixel(240., 1., 0.5),
            Pixel::from_normalized(0., 0., 1.)
        );

        assert_eq!(
            hsl_to_pixel(300., 1., 0.5),
            Pixel::from_normalized(1., 0., 1.)
        );

        assert_eq!(
            hsl_to_pixel(360., 1., 0.5),
            Pixel::from_normalized(1., 0., 0.)
        );

        assert_eq!(hsl_to_pixel(123., 0.35, 0.69), Pixel::rgb(148, 204, 151));
    }
}
