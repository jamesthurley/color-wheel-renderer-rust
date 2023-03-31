use crate::{
    bucket::bucket, color_wheel_definition::ColorWheelDefinition,
    get_angle_degrees::get_angle_degrees,
    get_pixel_generator_and_variable_dimension::GetPixelGeneratorAndVariableDimension,
    pixel_generators::pixel_generator::PixelGenerator, pixel_writer::PixelWriter,
};

pub struct RenderPixelData {
    pub center_x: usize,
    pub center_y: usize,
    pub edge_distance: f64,
    pub generator_size: f64,
}

pub trait RenderPixel {
    fn execute<TPixelGenerator: PixelGenerator, TPixelWriter: PixelWriter>(
        &self,
        image_x: usize,
        image_y: usize,
        data: &RenderPixelData,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    );
}

struct DefaultRenderPixel<TGetPixelGeneratorAndVariableDimension>
where
    TGetPixelGeneratorAndVariableDimension: GetPixelGeneratorAndVariableDimension,
{
    get_pixel_generator_and_variable_dimension: TGetPixelGeneratorAndVariableDimension,
}

impl<TGetPixelGeneratorAndVariableDimension> RenderPixel
    for DefaultRenderPixel<TGetPixelGeneratorAndVariableDimension>
where
    TGetPixelGeneratorAndVariableDimension: GetPixelGeneratorAndVariableDimension,
{
    fn execute<TPixelGenerator: PixelGenerator, TPixelWriter: PixelWriter>(
        &self,
        image_x: usize,
        image_y: usize,
        data: &RenderPixelData,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    ) {
        let relative_x = image_x - data.center_x;
        let relative_y = image_y - data.center_y;
        let distance_from_center = ((relative_x.pow(2) + relative_y.pow(2)) as f64).sqrt();
        if distance_from_center > data.edge_distance {
            return;
        }
        let pixel_generator_result = self.get_pixel_generator_and_variable_dimension.execute(
            data.generator_size,
            definition,
            distance_from_center,
        );
        if pixel_generator_result.is_none() {
            return;
        }
        let pixel_generator_result = pixel_generator_result.unwrap();
        let pixel_generator = pixel_generator_result.pixel_generator;
        let mut variable_dimension = pixel_generator_result.variable_dimension;
        let mut angle_degrees = get_angle_degrees(0., 0., relative_x as f64, relative_y as f64);
        variable_dimension = bucket(
            variable_dimension,
            1.,
            definition.distance_buckets,
            pixel_generator.varying_dimension_bucket_direction(),
        );
        angle_degrees = bucket(
            angle_degrees,
            360.,
            definition.angle_buckets,
            pixel_generator.angle_bucket_direction(),
        );
        if pixel_generator.is_varying_dimension_inverted() {
            variable_dimension = 1. - variable_dimension;
        }
        if pixel_generator.is_angle_inverted() {
            angle_degrees = 360. - angle_degrees;
        }
        let pixel = pixel_generator.get_pixel(angle_degrees, variable_dimension);

        if let Some(pixel) = pixel {
            pixel_writer.write_pixel(image_x, image_y, pixel);
        }
    }
}
