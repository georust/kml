pub mod types;

#[cfg(feature = "geo-types")]
pub mod conversion;

use crate::types::LineString;
use crate::types::LinearRing;
use crate::types::Point;
use crate::types::Polygon;

pub mod errors;
pub use crate::errors::Error;

#[non_exhaustive]
pub enum KmlVersion {
    Unknown,
    V22,
}

impl Default for KmlVersion {
    fn default() -> KmlVersion {
        KmlVersion::Unknown
    }
}

#[non_exhaustive]
pub enum KmlAlt {
    Feature,
    Geometry,
}

// TODO: Only start with abstract group for Feature, Geometry
// TODO: Should Kml contain every tag, with variants specifying more specific elements based on spec?
#[non_exhaustive]
pub enum Kml {
    Point(Point),
    LineString(LineString),
    LinearRing(LinearRing),
    Polygon(Polygon),
    MultiGeometry(Vec<Kml>),
}

pub struct KmlDocument {
    pub version: KmlVersion,
    pub elements: Vec<Kml>,
}
