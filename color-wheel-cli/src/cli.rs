use clap::{command, CommandFactory, Parser, ValueEnum};
use std::{ffi::OsStr, path::PathBuf};

use crate::OUTPUT_FILE_EXTENSION;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub color_wheel_type: ColorWheelType,

    /// Number of angular buckets to divide colors into. Defaults to 0, which gives a smooth output.
    #[arg(short, long, value_name = "COUNT", default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..=1000))]
    pub angular_buckets: u32,

    /// Number of radial buckets to divide colors into. Defaults to 0, which gives a smooth output.
    #[arg(short, long, value_name = "COUNT", default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..=1000))]
    pub radial_buckets: u32,

    /// Fixed values at which to render. Can be specified multiple times. Defaults to 0.5 for lightness or 1 for saturation and value.
    #[arg(short, long, value_name = "NUMBER")]
    pub fixed: Vec<f64>,

    /// Path to file where color wheel should be saved.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Diameter of the color wheel in pixels.
    #[arg(short, long, value_name = "PIXELS", default_value_t = 980, value_parser = clap::value_parser!(u32).range(5..))]
    pub diameter: u32,

    /// Size of margin around color wheel in pixels.
    #[arg(short, long, value_name = "PIXELS", default_value_t = 10)]
    pub margin: u32,

    /// Add additional wheels to outside of previous wheel.
    #[arg(short, long)]
    pub expand: bool,

    /// Reverses the order of colours from the center to edge of the wheel.
    #[arg(short = 'c', long)]
    pub reverse_radial_colors: bool,

    /// Reverses the direction of radial bucketing from the default. Defaults to outwards, or inwards if colors are reversed.
    #[arg(short = 'b', long)]
    pub reverse_radial_bucketing: bool,

    // How many times supersampled should the output be. Default is 2x supersampling.
    #[arg(short, long, value_name = "COUNT", default_value_t = 4, value_parser = clap::value_parser!(u32).range(1..=8))]
    pub supersampling: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ColorWheelType {
    HslFixedSaturation,
    HslFixedLightness,
    HsvFixedSaturation,
    HsvFixedValue,
    OklabFixedLightness,
    OklabFixedChroma,
}

pub fn process_cli_options(cli: Cli) -> Cli {
    let mut fixed = cli.fixed.clone();
    if fixed.is_empty() {
        fixed = match cli.color_wheel_type {
            ColorWheelType::HslFixedLightness => vec![0.5],
            _ => vec![1.0],
        };
    }

    if let Some(output_value) = cli.output.clone() {
        if output_value.extension() != Some(OsStr::new(OUTPUT_FILE_EXTENSION)) {
            let mut cmd = Cli::command();
            cmd.error(
                clap::error::ErrorKind::InvalidValue,
                format!("Output file must end in {OUTPUT_FILE_EXTENSION}"),
            )
            .exit();
        }
    }

    Cli { fixed, ..cli }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
