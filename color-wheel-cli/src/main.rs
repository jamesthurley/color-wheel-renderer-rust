use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use cli::{process_cli_options, Cli};

use generate_color_wheel::generate_color_wheel;

mod cli;
mod create_pixel_generator;
mod create_pixel_generator_configuration;
mod generate_color_wheel;

fn main() {
    let cli = process_cli_options(Cli::parse());

    let now = Instant::now();
    let pixel_writer = generate_color_wheel(&cli);
    println!("{}ms", now.elapsed().as_millis());

    let ppm = pixel_writer.canvas.get_ppm();
    let output_file_path = cli.output.unwrap_or(PathBuf::from("output.ppm"));
    std::fs::write(output_file_path, ppm).expect("Failed to write canvas.");
}
