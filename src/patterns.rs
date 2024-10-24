pub trait DataGenerator: Send + Sync {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64;
}

pub struct GradientPattern;
impl DataGenerator for GradientPattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        (x + y + band) as f64
    }
}

pub struct SineWavePattern;
impl DataGenerator for SineWavePattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        let fx = x as f64 / 50.0;
        let fy = y as f64 / 50.0;
        let phase = band as f64 * std::f64::consts::PI / 4.0;
        (fx.sin() + fy.cos() + phase.sin()) * 128.0 + 128.0
    }
}

pub struct NoisePattern;
impl DataGenerator for NoisePattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        (x as f64 * 12.9898 + y as f64 * 78.233 + band as f64 * 37.719).sin() * 43758.5453
    }
}
