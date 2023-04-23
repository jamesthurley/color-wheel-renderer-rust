use color_wheel_lib::{
    bucket::BucketDirection,
    canvas_pixel_writer::DefaultCanvasPixelWriter,
    canvas_pixel_writer_factory::DefaultCanvasPixelWriterFactory,
    color_wheel_definition::ColorWheelDefinition,
    get_pixel::DefaultGetPixel,
    get_pixel_generator_and_variable_dimension::DefaultGetPixelGeneratorAndVariableDimension,
    pixel::Pixel,
    pixel_generators::{
        HslFixedLightnessPixelGenerator, HslFixedSaturationPixelGenerator,
        HsvFixedSaturationPixelGenerator, PixelGeneratorConfiguration,
    },
    render_color_wheel::DefaultRenderColorWheel,
    render_color_wheel_set::DefaultRenderColorWheelSet,
    render_color_wheel_set::RenderColorWheelSet,
    render_pixel::DefaultRenderPixel,
};

fn setup() -> impl RenderColorWheelSet<Result = DefaultCanvasPixelWriter> {
    DefaultRenderColorWheelSet {
        render_color_wheel: DefaultRenderColorWheel {
            render_pixel: DefaultRenderPixel {
                get_pixel_generator_and_variable_dimension:
                    DefaultGetPixelGeneratorAndVariableDimension {},
                get_pixel: DefaultGetPixel {},
            },
        },
        pixel_writer_factory: DefaultCanvasPixelWriterFactory {},
    }
}

#[allow(dead_code)]
fn write(pixel_writer: DefaultCanvasPixelWriter) {
    std::fs::write("./output.ppm", pixel_writer.canvas.get_ppm()).expect("Failed to write canvas.");
}

#[test]
fn hsv_fixed_saturation_nested() {
    let render_color_wheel_set = setup();

    let configuration = PixelGeneratorConfiguration {
        is_angle_inverted: false,
        is_varying_dimension_inverted: false,
        angle_bucket_direction: BucketDirection::Down,
        varying_dimension_bucket_direction: BucketDirection::Down,
    };

    let definition = ColorWheelDefinition {
        image_size: 31,
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

    let pixel_writer = render_color_wheel_set.execute(&[definition], 0);

    assert_eq!(pixel_writer.canvas.get_pixel(15, 1), Pixel::rgb(170, 0, 0));
    assert_eq!(
        pixel_writer.canvas.get_pixel(15, 6),
        Pixel::rgb(170, 85, 85)
    );
    assert_eq!(
        pixel_writer.canvas.get_pixel(15, 24),
        Pixel::rgb(85, 170, 170)
    );
    assert_eq!(
        pixel_writer.canvas.get_pixel(10, 15),
        Pixel::rgb(195, 175, 234)
    );
    assert_eq!(
        pixel_writer.canvas.get_pixel(20, 15),
        Pixel::rgb(214, 234, 175)
    );

    // write(pixel_writer);
}

#[test]
fn hsl_fixed_saturation_adjacent() {
    let render_color_wheel_set = setup();

    let configuration = PixelGeneratorConfiguration {
        is_angle_inverted: false,
        is_varying_dimension_inverted: true,
        angle_bucket_direction: BucketDirection::Down,
        varying_dimension_bucket_direction: BucketDirection::Down,
    };

    let definition1 = ColorWheelDefinition {
        image_size: 31,
        margin_size: 0,
        angle_buckets: 18,
        distance_buckets: 12,
        pixel_generators: vec![HslFixedSaturationPixelGenerator {
            saturation: 1.0,
            configuration,
        }],
    };

    let definition2 = ColorWheelDefinition {
        image_size: 31,
        margin_size: 0,
        angle_buckets: 18,
        distance_buckets: 12,
        pixel_generators: vec![HslFixedSaturationPixelGenerator {
            saturation: 0.5,
            configuration,
        }],
    };

    let definition3 = ColorWheelDefinition {
        image_size: 31,
        margin_size: 0,
        angle_buckets: 18,
        distance_buckets: 12,
        pixel_generators: vec![HslFixedSaturationPixelGenerator {
            saturation: 0.25,
            configuration,
        }],
    };

    let pixel_writer = render_color_wheel_set.execute(&[definition1, definition2, definition3], 1);

    assert_eq!(pixel_writer.canvas.get_pixel(15, 1), Pixel::rgb(85, 0, 0));
    assert_eq!(pixel_writer.canvas.get_pixel(47, 1), Pixel::rgb(64, 21, 21));
    assert_eq!(pixel_writer.canvas.get_pixel(79, 1), Pixel::rgb(53, 32, 32));

    // write(pixel_writer);
}

#[test]
fn hsl_fixed_lightness_adjacent_margin() {
    let render_color_wheel_set = setup();

    let configuration = PixelGeneratorConfiguration {
        is_angle_inverted: false,
        is_varying_dimension_inverted: false,
        angle_bucket_direction: BucketDirection::Down,
        varying_dimension_bucket_direction: BucketDirection::Down,
    };

    let definition1 = ColorWheelDefinition {
        image_size: 31,
        margin_size: 2,
        angle_buckets: 18,
        distance_buckets: 12,
        pixel_generators: vec![HslFixedLightnessPixelGenerator {
            lightness: 0.6,
            configuration,
        }],
    };

    let definition2 = ColorWheelDefinition {
        image_size: 31,
        margin_size: 4,
        angle_buckets: 18,
        distance_buckets: 12,
        pixel_generators: vec![HslFixedLightnessPixelGenerator {
            lightness: 0.4,
            configuration,
        }],
    };

    let pixel_writer = render_color_wheel_set.execute(&[definition1, definition2], 1);

    assert_eq!(pixel_writer.canvas.get_pixel(15, 1), Pixel::transparent());
    assert_eq!(
        pixel_writer.canvas.get_pixel(15, 2),
        Pixel::rgb(247, 59, 59)
    );
    assert_eq!(pixel_writer.canvas.get_pixel(47, 3), Pixel::transparent());
    assert_eq!(pixel_writer.canvas.get_pixel(47, 4), Pixel::rgb(196, 8, 8));

    // write(pixel_writer);
}
