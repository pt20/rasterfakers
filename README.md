# RasterFakeRS

A Rust library and CLI tool for generating fake GeoTIFF files. Perfect for testing GIS applications, creating fixtures, and generating sample raster data.

## Features

- Generate GeoTIFF files with customizable dimensions, bands, and data types
- Multiple built-in data patterns (gradient, sine wave, noise)
- Support for custom data generation patterns
- Configurable parameters (projection, transform)
- Available as both a library and CLI tool
- Supports various data types (u8, u16, i16, u32, i32, f32, f64)

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
rasterfakers = "0.1.0"
```

### As a CLI Tool

```bash
cargo install rasterfakers
```

Note: RasterFakers depends on the GDAL library. Please ensure GDAL is installed on your system.

## Usage

### Library Usage

```rust
use rasterfakers::{FakeGeoTiffBuilder, GeoTransform, SineWavePattern};

// Create a GeoTIFF with default settings
let geotiff = FakeGeoTiffBuilder::new()
    .dimensions(256, 256)?
    .bands(1)?
    .projection("EPSG:4326")
    .geotransform(GeoTransform::default())
    .output_path("output.tiff")
    .data_generator(Box::new(SineWavePattern))
    .build::<f32>()?;

geotiff.write()?;
```

### CLI Usage

```bash
# Generate a basic GeoTIFF
rasterfakers -o output.tiff

# Customize dimensions and data type
rasterfakers -o custom.tiff -w 512 -e 512 -t f32

# Specify projection and resolution
rasterfakers -o projected.tiff -p "EPSG:4326" -r "0.1,0.1" -c "30.0,10.0"

# Use different data pattern
rasterfakers -o sine_pattern.tiff -n sine
```

## CLI Options

```bash
Options:
  -o, --output <PATH>                 Output file path
  -w, --width <N>                     Width of the GeoTIFF [default: 256]
  -e, --height <N>                    Height of the GeoTIFF [default: 256]
  -b, --bands <N>                     Number of bands [default: 1]
  -t, --data-type <TYPE>             Data type (u8, u16, i16, u32, i32, f32, f64) [default: f64]
  -p, --projection <PROJ>            Projection (e.g., EPSG:4326) [default: EPSG:4326]
  -r, --pixel-resolution <RES>       Pixel resolution (e.g., "0.25,0.25") [default: "1.0,1.0"]
  -c, --upper-left-corner <COORDS>   Upper-left corner coordinates [default: "0.0,0.0"]
  -n, --pattern <PATTERN>            Data pattern (gradient, sine, noise) [default: gradient]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

Check out the [examples](examples/) directory for more detailed usage examples:

- `examples/basic_usage.rs`: Simple library usage
- `examples/custom_pattern.rs`: Creating custom data patterns
- `examples/cli_examples.sh`: Various CLI usage examples

## Custom Data Patterns

You can create custom data patterns by implementing the `DataGenerator` trait:

```rust
use rasterfakers::DataGenerator;

struct CustomPattern;

impl DataGenerator for CustomPattern {
    fn generate(&self, x: usize, y: usize, band: usize) -> f64 {
        // Your custom pattern logic here
        (x * y * band) as f64
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
