use super::*;

const PPM_MAX_LINE_LENGTH: usize = 70;
const MAX_COLOR_ELEMENT_LENGTH: usize = 4;

impl Canvas {
    pub fn get_ppm(&self) -> String {
        let mut content = String::new();
        content.push_str("P3\n");
        content.push_str(&self.width.to_string());
        content.push(' ');
        content.push_str(&self.height.to_string());
        content.push('\n');
        content.push_str("255\n");

        for y in 0..self.height {
            let mut is_first_value_on_line = true;
            let mut line_length = 0;
            for x in 0..self.width {
                let pixel: Pixel = self.get_pixel(x, y);
                let values = [pixel.red(), pixel.green(), pixel.blue()];

                for value in values {
                    if (line_length + MAX_COLOR_ELEMENT_LENGTH) > PPM_MAX_LINE_LENGTH {
                        content.push('\n');
                        is_first_value_on_line = true;
                        line_length = 0;
                    }

                    if !is_first_value_on_line {
                        content.push(' ');
                        line_length += 1;
                    }

                    is_first_value_on_line = false;
                    let value_as_string = value.to_string();

                    content.push_str(&value_as_string);
                    line_length += value_as_string.len();
                }
            }
            content.push('\n');
        }

        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_write_header() {
        let target = canvas(5, 3);

        let ppm = target.get_ppm();

        let header_lines: Vec<&str> = ppm.lines().take(3).collect();

        assert_eq!(header_lines[0], "P3");
        assert_eq!(header_lines[1], "5 3");
        assert_eq!(header_lines[2], "255");
    }

    #[test]
    fn it_should_write_pixel_data() {
        let mut canvas = canvas(5, 3);
        let c1 = Pixel::from_normalized(1.5, 0.0, 0.0);
        let c2 = Pixel::from_normalized(0.0, 0.5, 0.0);
        let c3 = Pixel::from_normalized(-0.5, 0.0, 1.0);

        canvas.set_pixel(0, 0, c1);
        canvas.set_pixel(2, 1, c2);
        canvas.set_pixel(4, 2, c3);

        let ppm = canvas.get_ppm();

        let body_lines: Vec<&str> = ppm.lines().skip(3).collect();

        assert_eq!(body_lines.len(), 3);
        assert_eq!(body_lines[0], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(body_lines[1], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(body_lines[2], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn it_should_split_long_lines() {
        let mut canvas = canvas(10, 2);
        for x in 0..canvas.width {
            for y in 0..canvas.height {
                canvas.set_pixel(x, y, Pixel::from_normalized(1.0, 0.8, 0.6));
            }
        }

        let ppm = canvas.get_ppm();

        let body_lines: Vec<&str> = ppm.lines().skip(3).collect();

        assert_eq!(body_lines.len(), 4);
        assert_eq!(
            body_lines[0],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            body_lines[1],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            body_lines[2],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            body_lines[3],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    pub fn it_should_end_with_newline_character() {
        let canvas = canvas(5, 3);

        let ppm = canvas.get_ppm();

        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
