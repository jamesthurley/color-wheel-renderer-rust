use std::time::Instant;

use color_wheel_renderer::{
    bucket::BucketDirection,
    canvas::canvas_pixel_writer_factory::CanvasPixelWriterFactory,
    color_wheel_definition::ColorWheelDefinition,
    get_pixel::DefaultGetPixel,
    get_pixel_generator_and_variable_dimension::DefaultGetPixelGeneratorAndVariableDimension,
    pixel_generators::{HsvFixedSaturationPixelGenerator, PixelGeneratorConfiguration},
    render_color_wheel::DefaultRenderColorWheel,
    render_color_wheel_set::DefaultRenderColorWheelSet,
    render_color_wheel_set::RenderColorWheelSet,
    render_pixel::DefaultRenderPixel,
};

fn main() {
    let render_color_wheel_set = DefaultRenderColorWheelSet {
        render_color_wheel: DefaultRenderColorWheel {
            render_pixel: DefaultRenderPixel {
                get_pixel_generator_and_variable_dimension:
                    DefaultGetPixelGeneratorAndVariableDimension {},
                get_pixel: DefaultGetPixel {},
            },
        },
        pixel_writer_factory: CanvasPixelWriterFactory {},
    };

    let configuration = PixelGeneratorConfiguration {
        is_angle_inverted: false,
        is_varying_dimension_inverted: false,
        angle_bucket_direction: BucketDirection::Down,
        varying_dimension_bucket_direction: BucketDirection::Down,
    };

    let definition = ColorWheelDefinition {
        image_size: 2000,
        margin_size: 0,
        angle_buckets: 18,
        distance_buckets: 12,
        pixel_generators: vec![
            HsvFixedSaturationPixelGenerator {
                saturation: 0.25,
                configuration,
            },
            HsvFixedSaturationPixelGenerator {
                saturation: 0.5,
                configuration,
            },
            HsvFixedSaturationPixelGenerator {
                saturation: 1.0,
                configuration,
            },
        ],
    };

    let now = Instant::now();
    let pixel_writer = render_color_wheel_set.execute(&[definition], 0);
    println!("{}ms", now.elapsed().as_millis());

    let ppm = pixel_writer.canvas.get_ppm();

    std::fs::write("./output.ppm", ppm).expect("Failed to write canvas.");
}
