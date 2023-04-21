use color_wheel_lib::pixel_generators::{
    HslFixedLightnessPixelGenerator, HslFixedSaturationPixelGenerator,
    HsvFixedSaturationPixelGenerator, HsvFixedValuePixelGenerator, PixelGenerator,
    PixelGeneratorConfiguration,
};

pub trait CreatePixelGenerator<T: PixelGenerator> {
    fn execute(&self, fixed: f64, configuration: PixelGeneratorConfiguration) -> T;
}

pub struct DefaultCreatePixelGenerator {}

impl CreatePixelGenerator<HslFixedSaturationPixelGenerator> for DefaultCreatePixelGenerator {
    fn execute(
        &self,
        fixed: f64,
        configuration: PixelGeneratorConfiguration,
    ) -> HslFixedSaturationPixelGenerator {
        HslFixedSaturationPixelGenerator {
            saturation: fixed,
            configuration,
        }
    }
}

impl CreatePixelGenerator<HslFixedLightnessPixelGenerator> for DefaultCreatePixelGenerator {
    fn execute(
        &self,
        fixed: f64,
        configuration: PixelGeneratorConfiguration,
    ) -> HslFixedLightnessPixelGenerator {
        HslFixedLightnessPixelGenerator {
            lightness: fixed,
            configuration,
        }
    }
}

impl CreatePixelGenerator<HsvFixedSaturationPixelGenerator> for DefaultCreatePixelGenerator {
    fn execute(
        &self,
        fixed: f64,
        configuration: PixelGeneratorConfiguration,
    ) -> HsvFixedSaturationPixelGenerator {
        HsvFixedSaturationPixelGenerator {
            saturation: fixed,
            configuration,
        }
    }
}

impl CreatePixelGenerator<HsvFixedValuePixelGenerator> for DefaultCreatePixelGenerator {
    fn execute(
        &self,
        fixed: f64,
        configuration: PixelGeneratorConfiguration,
    ) -> HsvFixedValuePixelGenerator {
        HsvFixedValuePixelGenerator {
            value: fixed,
            configuration,
        }
    }
}
