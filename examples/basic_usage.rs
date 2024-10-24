use rasterfakers::FakeGeoTiffBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geotiff = FakeGeoTiffBuilder::new()
        .dimensions(256, 256)?
        .bands(1)?
        .projection("EPSG:4326")
        .output_path("basic_output.tiff")
        .build::<f32>()?;

    geotiff.write()?;
    println!("Basic GeoTIFF generated successfully!");
    Ok(())
}
