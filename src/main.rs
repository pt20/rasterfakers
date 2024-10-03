use rasterfakers::geotiff::{FakeGeoTiff, FakeGeoTiffConfig};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let config = FakeGeoTiffConfig {
        width: 256,
        height: 256,
        bands: 3,
        projection: Some("EPSG:4326".to_string()),
        pixel_resolution: Some((0.25, 0.25)),
        upper_left_corner: Some((30.0, 10.0)),
        output_path: Some("fake_geotiff_u16.tif".to_string()),
        ..Default::default()
    };

    FakeGeoTiff::<u16>::from_config(config).write_to_file()?;

    Ok(())
}
