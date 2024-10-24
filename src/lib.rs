pub mod conversions;
pub mod error;
pub mod geotiff;
pub mod patterns;

pub use error::{GeoTiffError, Result};
pub use geotiff::{FakeGeoTiff, FakeGeoTiffBuilder, GeoTransform};
pub use patterns::{DataGenerator, GradientPattern, NoisePattern, SineWavePattern};
