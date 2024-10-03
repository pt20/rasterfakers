use rasterfakers::geotiff::FakeGeoTiff;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Example with f64
    FakeGeoTiff::<f64>::new()
        .width(512)
        .height(512)
        .bands(1)
        .projection("EPSG:4326")
        .pixel_resolution(0.00025, 0.00025) // Set pixel width and height
        .upper_left_corner(30.0, 10.0) // Set the top-left corner coordinates (x_min, y_max)
        .write_to_file("fake_geotiff_f64.tif")?;

    // Example with u16
    FakeGeoTiff::<u16>::new()
        .width(256)
        .height(256)
        .bands(3)
        .projection("EPSG:4326")
        .pixel_resolution(0.25, 0.25) // Set pixel width and height
        .upper_left_corner(30.0, 10.0) // Set the top-left corner coordinates (x_min, y_max)
        .write_to_file("fake_geotiff_u16.tif")?;

    Ok(())
}
