use rasterfakers::{FakeGeoTiffBuilder, GeoTiffError};

#[test]
fn test_zero_dimensions() {
    let result = FakeGeoTiffBuilder::new().dimensions(0, 100);
    match result {
        Err(GeoTiffError::InvalidDimensions(msg)) => {
            assert_eq!(msg, "Width and height must be greater than 0");
        }
        _ => panic!("Expected InvalidDimensions error for zero width"),
    }

    let result = FakeGeoTiffBuilder::new().dimensions(100, 0);
    match result {
        Err(GeoTiffError::InvalidDimensions(msg)) => {
            assert_eq!(msg, "Width and height must be greater than 0");
        }
        _ => panic!("Expected InvalidDimensions error for zero height"),
    }
}

#[test]
fn test_zero_bands() {
    let result = FakeGeoTiffBuilder::new()
        .dimensions(100, 100)
        .unwrap()
        .bands(0);
    assert!(result.is_err());
    match result {
        Err(GeoTiffError::InvalidDimensions(msg)) => {
            assert_eq!(msg, "Number of bands must be greater than 0");
        }
        _ => panic!("Expected InvalidDimensions error for zero bands"),
    }
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
    match result {
        Err(GeoTiffError::MissingField(msg)) => {
            assert_eq!(msg, "Output path must be specified");
        }
        _ => panic!("Expced MissingField error for missing output path"),
    }
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
