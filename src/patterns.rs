/// Trait for generating data patterns in GeoTIFFs.
///
/// Implement this trait to create custom data generation patterns for GeoTIFFs.
///
/// # Examples
///
/// Creating a custom checkerboard pattern:
///
/// ```
/// use rasterfakers::DataGenerator;
///
/// struct CheckerboardPattern;
///
/// impl DataGenerator for CheckerboardPattern {
///     fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
///         if (x + y + band) % 2 == 0 {
///             255.0
///         } else {
///             0.0
///         }
///     }
/// }
/// ```
pub trait DataGenerator: Send + Sync {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64;
}

/// A simple gradient pattern generator.
///
/// This pattern creates a gradient based on the sum of x, y, and band indices.
/// It produces a diagonal gradient effect across the image, with values
/// increasing from the top-left to the bottom-right corner.
///
/// # Examples
///
/// ```
/// use rasterfakers::{FakeGeoTiffBuilder, GradientPattern};
///
/// let geotiff = FakeGeoTiffBuilder::new()
///     .dimensions(256, 256).unwrap()
///     .bands(1).unwrap()
///     .data_generator(Box::new(GradientPattern))
///     .output_path("gradient.tiff")
///     .build::<u8>().unwrap();
///
/// geotiff.write().unwrap();
/// ```
pub struct GradientPattern;
impl DataGenerator for GradientPattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        (x + y + band) as f64
    }
}

/// A sine wave pattern generator.
///
/// This pattern creates a complex sine wave pattern using the x and y coordinates
/// and the band index. It produces an interesting interference pattern that
/// varies across bands.
///
/// # Examples
///
/// ```
/// use rasterfakers::{FakeGeoTiffBuilder, SineWavePattern};
///
/// let geotiff = FakeGeoTiffBuilder::new()
///     .dimensions(512, 512).unwrap()
///     .bands(3).unwrap()
///     .data_generator(Box::new(SineWavePattern))
///     .output_path("sine_wave.tiff")
///     .build::<f32>().unwrap();
///
/// geotiff.write().unwrap();
/// ```
pub struct SineWavePattern;
impl DataGenerator for SineWavePattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        let fx = x as f64 / 50.0;
        let fy = y as f64 / 50.0;
        let phase = band as f64 * std::f64::consts::PI / 4.0;
        (fx.sin() + fy.cos() + phase.sin()) * 128.0 + 128.0
    }
}

/// A noise pattern generator.
///
/// This pattern creates a pseudo-random noise pattern using a simple hash function.
/// It produces a different noise pattern for each band, giving a multi-band
/// noise effect.
///
/// # Examples
///
/// ```
/// use rasterfakers::{FakeGeoTiffBuilder, NoisePattern};
///
/// let geotiff = FakeGeoTiffBuilder::new()
///     .dimensions(1024, 1024).unwrap()
///     .bands(1).unwrap()
///     .data_generator(Box::new(NoisePattern))
///     .output_path("noise.tiff")
///     .build::<u16>().unwrap();
///
/// geotiff.write().unwrap();
/// ```
pub struct NoisePattern;
impl DataGenerator for NoisePattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        (x as f64 * 12.9898 + y as f64 * 78.233 + band as f64 * 37.719).sin() * 43758.5453
    }
}
