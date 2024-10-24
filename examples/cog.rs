use rasterfakers::{FakeGeoTiffBuilder, SineWavePattern};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geotiff = FakeGeoTiffBuilder::new()
        .dimensions(512, 512)?
        .bands(3)?
        .projection("EPSG:4326")
        .output_path("cloud_optimized.tiff")
        .data_generator(Box::new(SineWavePattern))
        .cloud_optimized(true)
        .build::<f32>()?;

    geotiff.write()?;
    println!("Cloud Optimized GeoTIFF generated successfully!");
    Ok(())
}
