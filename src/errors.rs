use thiserror::Error;

/// Errors for KML reading and writing
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid input supplied for XML")]
    InvalidInput,
    #[error("Encountered malformed XML: {0}")]
    MalformedXml(#[from] quick_xml::Error),
    #[error("Invalid XML event: {0}")]
    InvalidXmlEvent(String),
    #[error("Coordinate empty")]
    CoordEmpty,
    #[error("No KML elements found")]
    NoElements,
    #[error("Error parsing number from: {0}")]
    NumParse(String),
    #[error("Invalid KML version: {0}")]
    InvalidKmlVersion(String),
    #[error("Invalid KML element: {0}")]
    InvalidKmlElement(String),
    #[error("Geometry is invalid: {0}")]
    InvalidGeometry(String),
    #[error("Invalid altitude mode: {0}")]
    InvalidAltitudeMode(String),
    #[error("Invalid color mode: {0}")]
    InvalidColorMode(String),
    #[error("Invalid list item type: {0}")]
    InvalidListItemType(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("ZIP error: {0}")]
    ZipError(#[from] zip::result::ZipError),
}
