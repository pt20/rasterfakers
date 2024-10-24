use crate::conversions::ConvertFromF64;
use crate::error::{GeoTiffError, Result};
use crate::patterns::DataGenerator;
use gdal::raster::Buffer;
use gdal::raster::GdalType;
use gdal::raster::RasterCreationOptions;
use gdal::DriverManager;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GeoTransform {
    pub x_min: f64,
    pub pixel_width: f64,
    pub rotation_x: f64,
    pub y_max: f64,
    pub rotation_y: f64,
    pub pixel_height: f64,
}

impl Default for GeoTransform {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            pixel_width: 1.0,
            rotation_x: 0.0,
            y_max: 0.0,
            rotation_y: 0.0,
            pixel_height: -1.0,
        }
    }
}

impl From<GeoTransform> for [f64; 6] {
    fn from(transform: GeoTransform) -> Self {
        [
            transform.x_min,
            transform.pixel_width,
            transform.rotation_x,
            transform.y_max,
            transform.rotation_y,
            transform.pixel_height,
        ]
    }
}

pub struct FakeGeoTiff<T>
where
    T: GdalType + Default + Clone + ConvertFromF64 + Copy + Send + Sync,
{
    width: usize,
    height: usize,
    bands: usize,
    projection: Option<String>,
    geotransform: Option<GeoTransform>,
    output_path: PathBuf,
    data_generator: Box<dyn DataGenerator>,
    cloud_optimized: bool,
    _phantom: std::marker::PhantomData<T>,
}

/// A builder for creating fake GeoTIFF files.
///
/// This struct provides a fluent interface for configuring and generating
/// fake GeoTIFF files with customizable properties such as dimensions, bands,
/// projection, and data generation patterns.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rasterfakers::{FakeGeoTiffBuilder, GradientPattern};
///
/// let geotiff = FakeGeoTiffBuilder::new()
///     .dimensions(256, 256).unwrap()
///     .bands(1).unwrap()
///     .projection("EPSG:4326")
///     .output_path("output.tiff")
///     .build::<f32>().unwrap();
///
/// geotiff.write().unwrap();
/// ```
///
/// Creating a multi-band GeoTIFF with a custom pattern:
///
/// ```
/// use rasterfakers::{FakeGeoTiffBuilder, DataGenerator};
///
/// struct CustomPattern;
/// impl DataGenerator for CustomPattern {
///     fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
///         (x + y + band) as f64 / 255.0
///     }
/// }
///
/// let geotiff = FakeGeoTiffBuilder::new()
///     .dimensions(512, 512).unwrap()
///     .bands(3).unwrap()
///     .projection("EPSG:3857")
///     .data_generator(Box::new(CustomPattern))
///     .output_path("custom_pattern.tiff")
///     .build::<u8>().unwrap();
///
/// geotiff.write().unwrap();
/// ```
pub struct FakeGeoTiffBuilder {
    width: usize,
    height: usize,
    bands: usize,
    projection: Option<String>,
    geotransform: Option<GeoTransform>,
    output_path: Option<PathBuf>,
    data_generator: Option<Box<dyn DataGenerator>>,
    cloud_optimized: bool,
}

impl Default for FakeGeoTiffBuilder {
    fn default() -> Self {
        Self {
            width: 256,
            height: 256,
            bands: 1,
            projection: None,
            geotransform: Some(GeoTransform::default()),
            output_path: None,
            data_generator: None,
            cloud_optimized: false,
        }
    }
}

impl FakeGeoTiffBuilder {
    /// Creates a new `FakeGeoTiffBuilder` with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rasterfakers::FakeGeoTiffBuilder;
    ///
    /// let builder = FakeGeoTiffBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dimensions(mut self, width: usize, height: usize) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(GeoTiffError::InvalidDimensions(
                "Width and height must be greater than 0".into(),
            ));
        }
        self.width = width;
        self.height = height;
        Ok(self)
    }

    pub fn bands(mut self, bands: usize) -> Result<Self> {
        if bands == 0 {
            return Err(GeoTiffError::InvalidDimensions(
                "Number of bands must be greater than 0".into(),
            ));
        }
        self.bands = bands;
        Ok(self)
    }

    pub fn projection(mut self, projection: impl Into<String>) -> Self {
        self.projection = Some(projection.into());
        self
    }

    pub fn geotransform(mut self, transform: GeoTransform) -> Self {
        self.geotransform = Some(transform);
        self
    }

    pub fn output_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.output_path = Some(path.into());
        self
    }

    pub fn data_generator(mut self, generator: Box<dyn DataGenerator>) -> Self {
        self.data_generator = Some(generator);
        self
    }

    pub fn cloud_optimized(mut self, cloud_optimized: bool) -> Self {
        self.cloud_optimized = cloud_optimized;
        self
    }

    /// Builds the `FakeGeoTiff` instance with the configured settings.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The data type for the GeoTIFF. Must implement `GdalType`,
    ///         `Default`, `Clone`, `ConvertFromF64`, `Copy`, `Send`, and `Sync`.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rasterfakers::FakeGeoTiffBuilder;
    ///
    /// let geotiff = FakeGeoTiffBuilder::new()
    ///     .dimensions(256, 256).unwrap()
    ///     .bands(1).unwrap()
    ///     .output_path("output.tiff")
    ///     .build::<f32>().unwrap();
    /// ```
    pub fn build<T>(self) -> Result<FakeGeoTiff<T>>
    where
        T: GdalType + Default + Clone + ConvertFromF64 + Copy + Send + Sync,
    {
        let output_path = self
            .output_path
            .ok_or_else(|| GeoTiffError::MissingField("Output path must be specified".into()))?;

        Ok(FakeGeoTiff {
            width: self.width,
            height: self.height,
            bands: self.bands,
            projection: self.projection,
            geotransform: self.geotransform,
            output_path,
            data_generator: self
                .data_generator
                .unwrap_or_else(|| Box::new(crate::patterns::GradientPattern)),
            cloud_optimized: self.cloud_optimized,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<T> FakeGeoTiff<T>
where
    T: GdalType + Default + Clone + ConvertFromF64 + Copy + Send + Sync,
{
    fn generate_data(&self) -> Vec<T> {
        let total_size = self.width * self.height * self.bands;
        let mut data = Vec::with_capacity(total_size);

        for band in 0..self.bands {
            for y in 0..self.height {
                for x in 0..self.width {
                    let value = self.data_generator.generate(x, y, band);
                    data.push(T::convert_from_f64(value));
                }
            }
        }

        data
    }

    pub fn write(&self) -> Result<()> {
        // Here we handle the cloud optimized part
        let creation_options = if self.cloud_optimized {
            vec![
                "TILED=YES",
                "COMPRESS=LZW",
                "COPY_SRC_OVERVIEWS=YES",
                "BIGTIFF=IF_SAFER",
            ]
        } else {
            vec![]
        };
        let options = RasterCreationOptions::from_iter(creation_options);

        let driver = DriverManager::get_driver_by_name("GTiff")?;
        let mut dataset = driver.create_with_band_type_with_options::<T, _>(
            &self.output_path,
            self.width,
            self.height,
            self.bands,
            &options,
        )?;

        if let Some(proj) = &self.projection {
            dataset.set_projection(proj)?;
        }

        if let Some(transform) = &self.geotransform {
            dataset.set_geo_transform(&Into::<[f64; 6]>::into(transform.clone()))?;
        }

        let data = self.generate_data();

        for band_index in 1..=self.bands {
            let mut band = dataset.rasterband(band_index)?;
            let start = (band_index - 1) * self.width * self.height;
            let end = band_index * self.width * self.height;
            let band_data = &data[start..end];

            let mut buffer = Buffer::new((self.width, self.height), band_data.to_vec());

            band.write((0, 0), (self.width, self.height), &mut buffer)?;
        }

        if self.cloud_optimized {
            // TIL: empty can be passed as &[] - and here I was going through std::io::empty spiral
            dataset.build_overviews("NEAREST", &[2, 4, 8, 16], &[])?;
        }

        Ok(())
    }
}
