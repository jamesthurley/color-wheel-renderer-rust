use crate::pixel_generators::PixelGenerator;

pub struct ColorWheelDefinition<TPixelGenerator>
where
    TPixelGenerator: PixelGenerator,
{
    pub image_size: u32,
    pub margin_size: u32,
    pub angle_buckets: u32,
    pub distance_buckets: u32,
    pub pixel_generators: Vec<TPixelGenerator>,
}
