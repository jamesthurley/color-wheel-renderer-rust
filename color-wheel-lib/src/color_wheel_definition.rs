use crate::pixel_generators::PixelGenerator;

pub struct ColorWheelDefinition<TPixelGenerator>
where
    TPixelGenerator: PixelGenerator,
{
    pub image_size: usize,
    pub margin_size: usize,
    pub angle_buckets: usize,
    pub distance_buckets: usize,
    pub pixel_generators: Vec<TPixelGenerator>,
}
