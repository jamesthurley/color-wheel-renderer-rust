use crate::OUTPUT_FILE_EXTENSION;
use std::{num::NonZeroU32, path::PathBuf};

use color_wheel_lib::canvas_pixel_writer::DefaultCanvasPixelWriter;

use crate::cli::Cli;
use fast_image_resize as fr;

pub fn write_output_file(cli: Cli, pixel_writer: DefaultCanvasPixelWriter) {
    let output_file_path = cli
        .output
        .unwrap_or(PathBuf::from(format!("output.{OUTPUT_FILE_EXTENSION}")));

    let canvas_width = pixel_writer.canvas.width();
    let canvas_height = pixel_writer.canvas.height();
    let rgba = pixel_writer.canvas.eject_data();

    if cli.supersampling == 1 {
        image::save_buffer(
            output_file_path,
            &rgba,
            canvas_width,
            canvas_height,
            image::ColorType::Rgba8,
        )
        .expect("Failed to write canvas.");
    } else {
        let output_width = canvas_width / cli.supersampling;
        let output_height = canvas_height / cli.supersampling;

        let mut src_image = fr::Image::from_vec_u8(
            NonZeroU32::new(canvas_width).unwrap(),
            NonZeroU32::new(canvas_height).unwrap(),
            rgba,
            fr::PixelType::U8x4,
        )
        .expect("Failed to load image data in fast_image_resize.");

        // We are using premultiplied alpha here, to avoid the (black) transparent pixel
        // RGB information leaking out when resizing.
        // https://en.wikipedia.org/wiki/Alpha_compositing#Straight_versus_premultiplied

        // Multiple RGB channels of source image by alpha channel
        // (not required for the Nearest algorithm)
        let alpha_mul_div = fr::MulDiv::default();
        alpha_mul_div
            .multiply_alpha_inplace(&mut src_image.view_mut())
            .unwrap();

        // Create container for data of destination image
        let dst_width = NonZeroU32::new(output_width).unwrap();
        let dst_height = NonZeroU32::new(output_height).unwrap();
        let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

        // Get mutable view of destination image data
        let mut dst_view = dst_image.view_mut();

        // Create Resizer instance and resize source image
        // into buffer of destination image
        let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Hamming));
        resizer.resize(&src_image.view(), &mut dst_view).unwrap();

        // Divide RGB channels of destination image by alpha
        alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

        image::save_buffer(
            output_file_path,
            dst_image.buffer(),
            output_width,
            output_height,
            image::ColorType::Rgba8,
        )
        .expect("Failed to write canvas.");
    }
}
