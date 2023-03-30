use crate::pixel_generators::pixel_generator::PixelGenerator;

pub struct ColorWheelDefinition<TPixelGenerator>
where
    TPixelGenerator: PixelGenerator,
{
    pub size: usize,
    pub margin_size: usize,
    pub angle_buckets: usize,
    pub distance_buckets: usize,
    pub pixel_generators: Vec<TPixelGenerator>,
}
