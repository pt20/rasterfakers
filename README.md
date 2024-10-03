# RasterFakers

**RasterFakers** is a Rust library and CLI tool designed to generate fake but valid GeoTIFF files for testing purposes and creating fixtures in geospatial applications.

## Features

- Customizable dimensions, data types, projections, and georeferencing information.
- Synthetic data generation with a default gradient pattern.
- Command-Line Interface for easy usage.

## Installation

### Library as Dependency

Add `rasterfakers` to your `Cargo.toml`:

```toml
[dependencies]
rasterfakers = "0.1.0"
```

### CLI Tool

Install via cargo:

```bash
cargo install rasterfakers
```

Note: RasterFakers depends on the GDAL library. Please ensure GDAL is installed on your system.

## Usage

### CLI

Generate a default GeoTIFF:

```bash
rasterfakers --output fake.tif
```

Generate a custom GeoTIFF:

```bash
rasterfakers \
    --output custom.tif \
    --width 512 \
    --height 512 \
    --bands 3 \
    --data-type u16 \
    --projection EPSG:3857 \
    --pixel-resolution "0.5,0.5" \
    --upper-left-corner "500000.0,2000000.0"
```

### Library

```rust
use rasterfakers::geotiff::{FakeGeoTiff, FakeGeoTiffConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = FakeGeoTiffConfig {
        width: 256,
        height: 256,
        bands: 1,
        projection: Some("EPSG:4326".to_string()),
        pixel_resolution: Some((0.00025, 0.00025)),
        upper_left_corner: Some((30.0, 10.0)),
        output_path: Some("fake_geotiff.tif".to_string()),
        ..Default::default()
    };

    FakeGeoTiff::<f64>::from_config(config).write_to_file()?;
    Ok(())
}
```

## License

This project is licensed under the MIT License.
