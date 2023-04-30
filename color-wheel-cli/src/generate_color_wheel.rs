use color_wheel_lib::{
    canvas_pixel_writer::DefaultCanvasPixelWriter,
    canvas_pixel_writer_factory::DefaultCanvasPixelWriterFactory,
    color_wheel_definition::ColorWheelDefinition,
    get_pixel::DefaultGetPixel,
    get_pixel_generator_and_variable_dimension::DefaultGetPixelGeneratorAndVariableDimension,
    pixel_generators::{
        HslFixedLightnessPixelGenerator, HslFixedSaturationPixelGenerator,
        HsvFixedSaturationPixelGenerator, HsvFixedValuePixelGenerator, PixelGenerator,
    },
    render_color_wheel::DefaultRenderColorWheel,
    render_color_wheel_rows::DefaultRenderColorWheelRows,
    render_color_wheel_set::{DefaultRenderColorWheelSet, RenderColorWheelSet},
    render_pixel::DefaultRenderPixel,
};

use crate::{
    cli::{Cli, ColorWheelType},
    create_pixel_generator::{CreatePixelGenerator, DefaultCreatePixelGenerator},
    create_pixel_generator_configuration::create_pixel_generator_configuration,
};

pub fn generate_color_wheel(cli: &Cli) -> DefaultCanvasPixelWriter {
    let create_pixel_generator = DefaultCreatePixelGenerator {};
    match cli.color_wheel_type {
        ColorWheelType::HslFixedSaturation => generate_specific_color_wheel::<
            HslFixedSaturationPixelGenerator,
            DefaultCreatePixelGenerator,
        >(cli, create_pixel_generator),

        ColorWheelType::HslFixedLightness => generate_specific_color_wheel::<
            HslFixedLightnessPixelGenerator,
            DefaultCreatePixelGenerator,
        >(cli, create_pixel_generator),

        ColorWheelType::HsvFixedSaturation => generate_specific_color_wheel::<
            HsvFixedSaturationPixelGenerator,
            DefaultCreatePixelGenerator,
        >(cli, create_pixel_generator),

        ColorWheelType::HsvFixedValue => generate_specific_color_wheel::<
            HsvFixedValuePixelGenerator,
            DefaultCreatePixelGenerator,
        >(cli, create_pixel_generator),
    }
}

fn generate_specific_color_wheel<TPixelGenerator, TCreatePixelGenerator>(
    cli: &Cli,
    create_pixel_generator: TCreatePixelGenerator,
) -> DefaultCanvasPixelWriter
where
    TPixelGenerator: PixelGenerator,
    TCreatePixelGenerator: CreatePixelGenerator<TPixelGenerator>,
{
    let image_size: u32 = cli.supersampling * (cli.diameter + (cli.margin * 2));

    let configuration = create_pixel_generator_configuration(cli);

    let color_wheel_definitions = if cli.expand {
        vec![ColorWheelDefinition {
            image_size,
            margin_size: cli.margin,
            angle_buckets: cli.angular_buckets,
            distance_buckets: cli.radial_buckets,
            pixel_generators: cli
                .fixed
                .iter()
                .map(|v| create_pixel_generator.execute(*v, configuration))
                .collect(),
        }]
    } else {
        cli.fixed
            .iter()
            .map(|v| ColorWheelDefinition {
                image_size,
                margin_size: cli.margin,
                angle_buckets: cli.angular_buckets,
                distance_buckets: cli.radial_buckets,
                pixel_generators: vec![create_pixel_generator.execute(*v, configuration)],
            })
            .collect()
    };

    let render_color_wheel_set = DefaultRenderColorWheelSet {
        render_color_wheel: DefaultRenderColorWheel {
            render_color_wheel_rows: DefaultRenderColorWheelRows {
                render_pixel: DefaultRenderPixel {
                    get_pixel_generator_and_variable_dimension:
                        DefaultGetPixelGeneratorAndVariableDimension {},
                    get_pixel: DefaultGetPixel {},
                },
            },
        },
        pixel_writer_factory: DefaultCanvasPixelWriterFactory {},
    };

    render_color_wheel_set.execute(&color_wheel_definitions, 0)
}
