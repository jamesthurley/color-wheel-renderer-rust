use std::cmp::min;

use crate::{
    color_wheel_definition::ColorWheelDefinition,
    pixel_generators::pixel_generator::PixelGenerator,
    pixel_writer::PixelWriter,
    render_pixel::{RenderPixel, RenderPixelData},
};

// We're putting the `PixelWriter` as a generic parameter on the `ColorWheelRenderer` trait
// so that we can more easily mock it in `ColorWheelSetRenderer`.
pub trait ColorWheelRenderer<TPixelWriter: PixelWriter> {
    fn render<TPixelGenerator: PixelGenerator>(
        &self,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    );
}

pub struct DefaultColorWheelRenderer<TRenderPixel>
where
    TRenderPixel: RenderPixel,
{
    render_pixel: TRenderPixel,
}

impl<TRenderPixel: RenderPixel, TPixelWriter: PixelWriter> ColorWheelRenderer<TPixelWriter>
    for DefaultColorWheelRenderer<TRenderPixel>
{
    fn render<TPixelGenerator: PixelGenerator>(
        &self,
        definition: &ColorWheelDefinition<TPixelGenerator>,
        pixel_writer: &mut TPixelWriter,
    ) {
        if definition.pixel_generators.is_empty() {
            return;
        }

        let image_width = definition.size;
        let image_height = definition.size;

        let wheel_diameter = min(image_width, image_height) - (definition.margin_size * 2);

        let center_x = image_width / 2;
        let center_y = image_height / 2;

        let edge_distance = wheel_diameter as f64 / 2.;

        let renderer_size = edge_distance / definition.pixel_generators.len() as f64;
        if renderer_size < 1. {
            panic!("Image is too small.");
        }

        let data = RenderPixelData {
            center_x,
            center_y,
            edge_distance,
            renderer_size,
        };

        for image_x in 0..image_width {
            for image_y in 0..image_height {
                self.render_pixel
                    .execute(image_x, image_y, &data, definition, pixel_writer);
            }
        }
    }
}
