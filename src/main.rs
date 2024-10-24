use clap::Parser;
use rasterfakers::{
    patterns::{GradientPattern, NoisePattern, SineWavePattern},
    DataGenerator, FakeGeoTiffBuilder, GeoTransform,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "RasterFakers")]
#[command(version = "0.2.0")]
#[command(about = "Generates fake GeoTIFF files", long_about = None)]
struct CliArgs {
    /// Output file path
    #[arg(short = 'o', long)]
    output: PathBuf,

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

    /// Data pattern (gradient, sine, noise)
    #[arg(short = 'n', long, default_value = "gradient")]
    pattern: String,

    /// COG flag
    #[arg(long, default_value_t = false)]
    cloud_optimized: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    // Parse pixel resolution and upper-left corner
    let (pixel_width, pixel_height) = parse_tuple(&args.pixel_resolution)?;
    let (x_min, y_max) = parse_tuple(&args.upper_left_corner)?;

    let geotransform = GeoTransform {
        x_min,
        pixel_width,
        rotation_x: 0.0,
        y_max,
        rotation_y: 0.0,
        pixel_height: -pixel_height,
    };

    let data_generator: Box<dyn DataGenerator> = match args.pattern.as_str() {
        "sine" => Box::new(SineWavePattern),
        "noise" => Box::new(NoisePattern),
        _ => Box::new(GradientPattern),
    };

    let builder = FakeGeoTiffBuilder::new()
        .dimensions(args.width, args.height)?
        .bands(args.bands)?
        .projection(args.projection)
        .geotransform(geotransform)
        .output_path(args.output.clone())
        .data_generator(data_generator)
        .cloud_optimized(args.cloud_optimized);

    match args.data_type.as_str() {
        "u8" => builder.build::<u8>()?.write()?,
        "u16" => builder.build::<u16>()?.write()?,
        "i16" => builder.build::<i16>()?.write()?,
        "u32" => builder.build::<u32>()?.write()?,
        "i32" => builder.build::<i32>()?.write()?,
        "f32" => builder.build::<f32>()?.write()?,
        "f64" => builder.build::<f64>()?.write()?,
        _ => return Err(format!("Unsupported data type: {}", args.data_type).into()),
    }

    println!(
        "GeoTIFF generated successfully at {}",
        args.output.display()
    );

    Ok(())
}

fn parse_tuple(s: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("Expected two comma-separated values, got '{}'", s).into());
    }
    let first = parts[0].trim().parse::<f64>()?;
    let second = parts[1].trim().parse::<f64>()?;
    Ok((first, second))
}
