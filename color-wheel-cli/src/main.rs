use std::time::Instant;

use clap::Parser;
use cli::{process_cli_options, Cli};

use generate_color_wheel::generate_color_wheel;
use write_output_file::write_output_file;

mod cli;
mod create_pixel_generator;
mod create_pixel_generator_configuration;
mod generate_color_wheel;
mod write_output_file;

const OUTPUT_FILE_EXTENSION: &str = "png";

fn main() {
    let cli = process_cli_options(Cli::parse());

    let now = Instant::now();
    let pixel_writer = generate_color_wheel(&cli);
    println!("Generate: {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    write_output_file(cli, pixel_writer);
    println!("Write: {}ms", now.elapsed().as_millis());
}
