use crate::OUTPUT_FILE_EXTENSION;
use std::path::PathBuf;

use color_wheel_lib::canvas_pixel_writer::DefaultCanvasPixelWriter;

use crate::cli::Cli;

pub fn write_output_file(cli: Cli, pixel_writer: DefaultCanvasPixelWriter) {
    let output_file_path = cli
        .output
        .unwrap_or(PathBuf::from(format!("output{OUTPUT_FILE_EXTENSION}")));

    let canvas_width = pixel_writer.canvas.width();
    let canvas_height = pixel_writer.canvas.height();
    let rgba = pixel_writer.canvas.eject_data();

    let mut image = image::RgbaImage::from_raw(canvas_width, canvas_height, rgba)
        .expect("Unable to create output image buffer.");

    if cli.supersampling != 1 {
        image = image::imageops::resize(
            &image,
            canvas_width / cli.supersampling,
            canvas_height / cli.supersampling,
            image::imageops::FilterType::Triangle,
        );
    }

    image::save_buffer(
        output_file_path,
        &image,
        image.width(),
        image.height(),
        image::ColorType::Rgba8,
    )
    .expect("Failed to write canvas.");
}
