use std::f64::consts::PI;

use libm::atan2;

pub fn get_angle_degrees(center_x: f64, center_y: f64, relative_x: f64, relative_y: f64) -> f64 {
    let angle_radians = PI + atan2(-relative_x + center_x, relative_y - center_y);
    (angle_radians * 180. / PI) % 360.
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use super::*;

    #[test]
    fn test_get_angle_degrees() {
        assert_approx_eq!(f64, 0., get_angle_degrees(0., 0., 0., -100.));
        assert_approx_eq!(f64, 0., get_angle_degrees(10., 10., 10., -100.));
        assert_approx_eq!(f64, 90., get_angle_degrees(10., 10., 100., 10.));
        assert_approx_eq!(f64, 180., get_angle_degrees(10., 10., 10., 50.));
        assert_approx_eq!(f64, 270., get_angle_degrees(10., 10., -20., 10.));

        assert_approx_eq!(f64, 45., get_angle_degrees(0., 0., 1., -1.));
        assert_approx_eq!(f64, 135., get_angle_degrees(0., 0., 1., 1.));
        assert_approx_eq!(f64, 225., get_angle_degrees(0., 0., -1., 1.));
        assert_approx_eq!(f64, 315., get_angle_degrees(0., 0., -1., -1.));
    }
}
