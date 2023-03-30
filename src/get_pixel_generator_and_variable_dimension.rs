use crate::{
    color_wheel_definition::ColorWheelDefinition, pixel_generators::pixel_generator::PixelGenerator,
};

pub struct PixelGeneratorAndVariableDimension<'a, TPixelGenerator: PixelGenerator> {
    pub pixel_generator: &'a TPixelGenerator,
    pub variable_dimension: f64,
}

pub trait GetPixelGeneratorAndVariableDimension {
    fn execute<'a, TPixelGenerator: PixelGenerator>(
        &self,
        renderer_size: f64,
        definition: &'a ColorWheelDefinition<TPixelGenerator>,
        distance_from_center: f64,
    ) -> Option<PixelGeneratorAndVariableDimension<'a, TPixelGenerator>>;
}

struct DefaultGetPixelGeneratorAndVariableDimension {}
impl GetPixelGeneratorAndVariableDimension for DefaultGetPixelGeneratorAndVariableDimension {
    fn execute<'a, TPixelGenerator: PixelGenerator>(
        &self,
        renderer_size: f64,
        definition: &'a ColorWheelDefinition<TPixelGenerator>,
        distance_from_center: f64,
    ) -> Option<PixelGeneratorAndVariableDimension<'a, TPixelGenerator>> {
        let mut variable_dimension = 1.;
        let mut renderer_inner_distance = 0.;
        let mut renderer_outer_distance = renderer_size;
        let mut pixel_generator = None;

        for current_pixel_generator in definition.pixel_generators.iter() {
            if distance_from_center < renderer_outer_distance {
                variable_dimension =
                    (distance_from_center - renderer_inner_distance) / renderer_size;
                pixel_generator = Some(current_pixel_generator);
                break;
            }

            renderer_inner_distance = renderer_outer_distance;
            renderer_outer_distance += renderer_size;
        }

        pixel_generator.map(|pixel_generator| PixelGeneratorAndVariableDimension {
            pixel_generator,
            variable_dimension,
        })
    }
}
