use rasterfakers::{DataGenerator, FakeGeoTiffBuilder};

struct ChessboardPattern;

impl DataGenerator for ChessboardPattern {
    fn generate(&self, x: usize, y: usize, _band: usize) -> f64 {
        if (x + y) % 2 == 0 {
            255.0
        } else {
            0.0
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let geotiff = FakeGeoTiffBuilder::new()
        .dimensions(512, 512)?
        .bands(1)?
        .output_path("chessboard.tiff")
        .data_generator(Box::new(ChessboardPattern))
        .build::<u8>()?;

    geotiff.write()?;
    println!("Chessboard pattern GeoTIFF generated!");
    Ok(())
}
