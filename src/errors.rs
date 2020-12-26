use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Placeholder")]
    PlaceholderError,
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
    FloatParse(String),
    #[error("Invalid KML version: {0}")]
    InvalidKmlVersion(String),
    #[error("Geometry is invalid")]
    InvalidGeometry,
}
