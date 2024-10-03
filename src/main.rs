use clap::Parser;
use rasterfakers::geotiff::{FakeGeoTiff, FakeGeoTiffConfig};
use std::error::Error;

/// A CLI tool to generate fake GeoTIFF files for testing and fixtures.
#[derive(Parser)]
#[command(name = "RasterFakers")]
#[command(version = "0.1.0")]
#[command(about = "Generates fake GeoTIFF files", long_about = None)]
struct CliArgs {
    /// Output file path
    #[arg(short = 'o', long)]
    output: String,

    /// Width of the GeoTIFF in pixels
    #[arg(short = 'w', long, default_value_t = 256)]
    width: usize,

    /// Height of the GeoTIFF in pixels - short is 'e' to avoid conflict with 'help'
    #[arg(short = 'e', long, default_value_t = 256)]
    height: usize,

    /// Number of bands
    #[arg(short = 'b', long, default_value_t = 1)]
    bands: usize,

    /// Data type (u8, u16, i16, u32, i32, f32, f64)
    #[arg(short = 't', long, default_value = "f64")]
    data_type: String,

    /// Projection (e.g., EPSG:4326)
    #[arg(short = 'p', long, default_value = "EPSG:4326")]
    projection: String,

    /// Pixel resolution as two comma-separated values (e.g., "0.25,0.25")
    #[arg(short = 'r', long, default_value = "1.0,1.0")]
    pixel_resolution: String,

    /// Upper-left corner coordinates as two comma-separated values (e.g., "30.0,10.0")
    #[arg(short = 'c', long, default_value = "0.0,0.0")]
    upper_left_corner: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let args = CliArgs::parse();

    // Parse pixel resolution
    let pixel_resolution = parse_tuple(&args.pixel_resolution)?;
    // Parse upper-left corner
    let upper_left_corner = parse_tuple(&args.upper_left_corner)?;

    // Create the configuration
    let config = FakeGeoTiffConfig {
        width: args.width,
        height: args.height,
        bands: args.bands,
        projection: Some(args.projection),
        pixel_resolution: Some(pixel_resolution),
        upper_left_corner: Some(upper_left_corner),
        output_path: Some(args.output.clone()),
        ..Default::default()
    };

    // Determine data type and generate the GeoTIFF
    match args.data_type.as_str() {
        "u8" => {
            FakeGeoTiff::<u8>::from_config(config).write_to_file()?;
        }
        "u16" => {
            FakeGeoTiff::<u16>::from_config(config).write_to_file()?;
        }
        "i16" => {
            FakeGeoTiff::<i16>::from_config(config).write_to_file()?;
        }
        "u32" => {
            FakeGeoTiff::<u32>::from_config(config).write_to_file()?;
        }
        "i32" => {
            FakeGeoTiff::<i32>::from_config(config).write_to_file()?;
        }
        "f32" => {
            FakeGeoTiff::<f32>::from_config(config).write_to_file()?;
        }
        "f64" => {
            FakeGeoTiff::<f64>::from_config(config).write_to_file()?;
        }
        _ => {
            eprintln!("Unsupported data type: {}", args.data_type);
            std::process::exit(1);
        }
    }

    println!("GeoTIFF generated successfully at {}", args.output);

    Ok(())
}

// Helper function to parse comma-separated values into a tuple of f64
fn parse_tuple(s: &str) -> Result<(f64, f64), Box<dyn Error>> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("Expected two comma-separated values, got '{}'", s).into());
    }
    let first = parts[0].trim().parse::<f64>()?;
    let second = parts[1].trim().parse::<f64>()?;
    Ok((first, second))
}
