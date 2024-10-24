use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeoTiffError {
    #[error("GDAL error: {0}")]
    Gdal(#[from] gdal::errors::GdalError),
    #[error("Invalid dimensions: {0}")]
    InvalidDimensions(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

pub type Result<T> = std::result::Result<T, GeoTiffError>;
