use crate::common::Pixel;

pub fn intermediate_cxm_to_pixel(hue: f64, c: f64, x: f64, m: f64) -> Pixel {
    let i = match hue {
        h if h < 60. => (c, x, 0.),
        h if h < 120. => (x, c, 0.),
        h if h < 180. => (0., c, x),
        h if h < 240. => (0., x, c),
        h if h < 300. => (x, 0., c),
        _ => (c, 0., x),
    };

    let r = i.0 + m;
    let g = i.1 + m;
    let b = i.2 + m;

    Pixel::from_normalized(r, g, b)
}
