use rasterfakers::FakeGeoTiffBuilder;

#[test]
fn test_zero_dimensions() {
    let result = FakeGeoTiffBuilder::new().dimensions(0, 100);
    assert!(result.is_err());

    let result = FakeGeoTiffBuilder::new().dimensions(100, 0);
    assert!(result.is_err());
}

#[test]
fn test_zero_bands() {
    let result = FakeGeoTiffBuilder::new()
        .dimensions(100, 100)
        .unwrap()
        .bands(0);
    assert!(result.is_err());
}

#[test]
fn test_missing_output_path() {
    let result = FakeGeoTiffBuilder::new()
        .dimensions(100, 100)
        .unwrap()
        .bands(1)
        .unwrap()
        .build::<f32>();
    assert!(result.is_err());
}

#[test]
fn test_valid_builder() {
    let result = FakeGeoTiffBuilder::new()
        .dimensions(100, 100)
        .unwrap()
        .bands(1)
        .unwrap()
        .output_path("test.tiff")
        .build::<f32>();
    assert!(result.is_ok());
}
