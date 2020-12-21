pub mod types;

#[cfg(feature = "geo-types")]
pub mod conversion;

pub use crate::types::Coord;
pub use crate::types::LineString;
pub use crate::types::LinearRing;
pub use crate::types::Point;
pub use crate::types::Polygon;
pub use crate::types::{Kml, KmlDocument, KmlVersion};

pub use crate::types::Feature;
pub use crate::types::Geometry;

pub mod errors;
pub use crate::errors::Error;

pub mod reader;
pub use crate::reader::KmlReader;
