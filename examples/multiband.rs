use rasterfakers::{FakeGeoTiffBuilder, SineWavePattern};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geotiff = FakeGeoTiffBuilder::new()
        .dimensions(512, 512)?
        .bands(3)?
        .projection("EPSG:4326")
        .output_path("multiband.tiff")
        .data_generator(Box::new(SineWavePattern))
        .build::<f32>()?;

    geotiff.write()?;
    println!("Multiband GeoTIFF generated successfully!");
    Ok(())
}
