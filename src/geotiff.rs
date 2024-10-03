use crate::conversions::ConvertFromF64;
use gdal::errors::Result as GdalResult;
use gdal::raster::Buffer;
use gdal::raster::GdalType;
use gdal::DriverManager;

pub struct FakeGeoTiffConfig {
    pub width: usize,
    pub height: usize,
    pub bands: usize,
    pub projection: Option<String>,
    pub pixel_resolution: Option<(f64, f64)>,
    pub upper_left_corner: Option<(f64, f64)>,
    pub geotransform: Option<[f64; 6]>,
    pub output_path: Option<String>,
}

impl Default for FakeGeoTiffConfig {
    fn default() -> Self {
        Self {
            width: 256,
            height: 256,
            bands: 1,
            projection: None,
            pixel_resolution: None,
            upper_left_corner: None,
            geotransform: None,
            output_path: None,
        }
    }
}

pub struct FakeGeoTiff<T>
where
    T: GdalType + Default + Clone + ConvertFromF64 + Copy,
{
    config: FakeGeoTiffConfig,
    data: Option<Vec<T>>,
}

impl<T> FakeGeoTiff<T>
where
    T: GdalType + Default + Clone + ConvertFromF64 + Copy,
{
    pub fn from_config(config: FakeGeoTiffConfig) -> Self {
        Self { config, data: None }
    }

    fn calculate_geotransform(&self) -> [f64; 6] {
        let x_min = self.config.upper_left_corner.map_or(0.0, |(x, _)| x);
        let y_max = self.config.upper_left_corner.map_or(0.0, |(_, y)| y);
        let (pixel_width, pixel_height) = self.config.pixel_resolution.unwrap_or((1.0, 1.0));
        [
            x_min,         // x_min (upper-left corner X)
            pixel_width,   // pixel_width (X resolution)
            0.0,           // rotation_x (always 0)
            y_max,         // y_max (upper-left corner Y)
            0.0,           // rotation_y (always 0)
            -pixel_height, // pixel_height (Y resolution, negative for top-down)
        ]
    }

    fn generate_gradient_data(&self) -> Vec<T> {
        let mut data =
            Vec::with_capacity(self.config.width * self.config.height * self.config.bands);
        for band in 0..self.config.bands {
            for y in 0..self.config.height {
                for x in 0..self.config.width {
                    let value = Self::calculate_value(x, y, band);
                    data.push(value);
                }
            }
        }
        data
    }

    fn calculate_value(x: usize, y: usize, band: usize) -> T {
        T::convert_from_f64((x + y + band) as f64)
    }

    pub fn write_to_file(mut self) -> GdalResult<()> {
        let path = self
            .config
            .output_path
            .as_ref()
            .expect("Output path must be set");

        // Calculate geotransform if not set
        if self.config.geotransform.is_none() {
            self.config.geotransform = Some(self.calculate_geotransform());
        }

        // Fill data if it's None
        if self.data.is_none() {
            let generated_data = self.generate_gradient_data();
            self.data = Some(generated_data);
        }

        let data = self.data.as_ref().unwrap();

        let driver = DriverManager::get_driver_by_name("GTiff")?;
        let mut dataset = driver.create_with_band_type::<T, _>(
            path,
            self.config.width,
            self.config.height,
            self.config.bands,
        )?;

        // Set projection
        if let Some(projection) = &self.config.projection {
            dataset.set_projection(projection)?;
        }

        // Set geotransform
        if let Some(geotransform) = &self.config.geotransform {
            dataset.set_geo_transform(geotransform)?;
        }

        // Write data to bands
        for band_index in 1..=self.config.bands {
            let mut band = dataset.rasterband(band_index)?;
            let start = (band_index - 1) * self.config.width * self.config.height;
            let end = band_index * self.config.width * self.config.height;
            let band_data = &data[start..end];
            let mut buffer =
                Buffer::new((self.config.width, self.config.height), band_data.to_vec());
            band.write((0, 0), (self.config.width, self.config.height), &mut buffer)?;
        }

        Ok(())
    }
}
