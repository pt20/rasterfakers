# RasterFakers Examples

This directory contains examples demonstrating how to use RasterFakers both as a library and as a CLI tool.

## Running Examples

### Rust Examples

You can run any Rust example using cargo:

```bash
# Run basic usage example
cargo run --example basic_usage

# Run custom pattern example
cargo run --example custom_pattern

# Run multiband example
cargo run --example multiband

# Run Cloud Optimized GeoTIFF example
cargo run --example cog
```

### CLI Examples

The `cli_examples.sh` script contains various CLI usage examples. Make sure the script is executable:

```bash
# Make the script executable
chmod +x cli_examples.sh

# Run the examples
./cli_examples.sh
```

## Example Descriptions

- `basic_usage.rs`: Demonstrates the simplest way to generate a GeoTIFF using the library
- `custom_pattern.rs`: Shows how to create and use a custom data pattern (chessboard pattern)
- `multiband.rs`: Examples of creating multi-band GeoTIFFs
- `cli_examples.sh`: Various command-line usage examples
- `cog.rs`: Demonstrates how to create a COG

## Prerequisites

Make sure you have installed RasterFakers either through cargo or by building it locally:

```bash
# Install from crates.io
cargo install rasterfakers

# Or build locally from the project root
cargo build --release
```
