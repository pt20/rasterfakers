use crate::conversions::ConvertFromF64;
use gdal::errors::Result as GdalResult;
use gdal::raster::Buffer;
use gdal::raster::GdalType;
use gdal::DriverManager;
use std::path::Path;

/// A struct representing a fake GeoTIFF with generic data type T.
pub struct FakeGeoTiff<T>
where
    T: GdalType + Default + Clone + ConvertFromF64 + Copy,
{
    width: usize,
    height: usize,
    bands: usize,
    pixel_width: Option<f64>,
    pixel_height: Option<f64>,
    x_min: Option<f64>,
    y_max: Option<f64>,
    projection: Option<String>,
    geotransform: Option<[f64; 6]>,
    data: Option<Vec<T>>,
}

#[allow(clippy::new_without_default)]
impl<T> FakeGeoTiff<T>
where
    T: GdalType + Default + Clone + ConvertFromF64 + Copy,
{
    /// Creates a new `FakeGeoTiff` with default values.
    pub fn new() -> Self {
        FakeGeoTiff {
            width: 256,
            height: 256,
            bands: 1,
            pixel_width: None,
            pixel_height: None,
            x_min: None,
            y_max: None,
            projection: None,
            geotransform: None,
            data: None,
        }
    }

    /// Set the width of the GeoTIFF.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the GeoTIFF.
    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Set the number of bands in the GeoTIFF.
    pub fn bands(mut self, bands: usize) -> Self {
        self.bands = bands;
        self
    }

    /// Set the projection string for the GeoTIFF.
    pub fn projection(mut self, projection: &str) -> Self {
        self.projection = Some(projection.to_string());
        self
    }

    /// Set the pixel resolution for the GeoTIFF.
    pub fn pixel_resolution(mut self, pixel_width: f64, pixel_height: f64) -> Self {
        self.pixel_width = Some(pixel_width);
        self.pixel_height = Some(pixel_height);
        self
    }

    /// Set the upper-left corner coordinates (x_min, y_max) for the GeoTIFF.
    pub fn upper_left_corner(mut self, x_min: f64, y_max: f64) -> Self {
        self.x_min = Some(x_min);
        self.y_max = Some(y_max);
        self
    }

    /// Automatically calculate the geotransform array based on the resolution and upper-left corner.
    fn calculate_geotransform(&self) -> [f64; 6] {
        [
            self.x_min.unwrap_or(0.0),         // x_min (upper-left corner X)
            self.pixel_width.unwrap_or(1.0),   // pixel_width (X resolution)
            0.0,                               // rotation_x (always 0)
            self.y_max.unwrap_or(0.0),         // y_max (upper-left corner Y)
            0.0,                               // rotation_y (always 0)
            -self.pixel_height.unwrap_or(1.0), // pixel_height (Y resolution, negative for top-down)
        ]
    }

    /// Set the geotransform directly (optional).
    pub fn geotransform(mut self, geotransform: [f64; 6]) -> Self {
        self.geotransform = Some(geotransform);
        self
    }

    /// Fill the synthetic gradient data.
    pub fn fill_gradient(mut self) -> Self {
        let mut data = Vec::with_capacity(self.width * self.height * self.bands);
        for band in 0..self.bands {
            for y in 0..self.height {
                for x in 0..self.width {
                    let value = Self::calculate_value(x, y, band);
                    data.push(value);
                }
            }
        }
        self.data = Some(data);
        self
    }

    /// A function to calculate a value based on x, y, and band.
    /// Eventually, this function could be replaced with something more sophisticated.
    /// Some ideas include
    ///  - random noise
    ///  - fractals
    ///  - perlin noise (noise crate seems to be interesting)
    fn calculate_value(x: usize, y: usize, band: usize) -> T {
        // Convert the sum to f64 and then to T using the custom trait.
        T::convert_from_f64((x + y + band) as f64)
    }

    /// Write the `FakeGeoTiff` to a file.
    pub fn write_to_file<P: AsRef<Path>>(mut self, path: P) -> GdalResult<()> {
        // Calculate geotransform if not already set
        if self.geotransform.is_none() {
            self.geotransform = Some(self.calculate_geotransform());
        }

        let driver = DriverManager::get_driver_by_name("GTiff")?;
        let mut dataset =
            driver.create_with_band_type::<T, _>(path, self.width, self.height, self.bands)?;

        // Set projection if available
        if let Some(projection) = &self.projection {
            dataset.set_projection(projection)?;
        }

        // Set geotransform if available
        if let Some(geotransform) = &self.geotransform {
            dataset.set_geo_transform(geotransform)?;
        }

        // Prepare data by owning the vector
        let data = self
            .data
            .unwrap_or_else(|| vec![T::default(); self.width * self.height * self.bands]);

        for band_index in 1..=self.bands {
            let mut band = dataset.rasterband(band_index)?;
            let band_data = &data[(band_index - 1) * self.width * self.height
                ..band_index * self.width * self.height];
            let mut buffer = Buffer::new((self.width, self.height), band_data.to_vec());
            band.write((0, 0), (self.width, self.height), &mut buffer)?;
        }

        Ok(())
    }
}
