use std::str::FromStr;

use num_traits::Float;

use crate::errors::Error;
use crate::types::{Element, LineString, LinearRing, MultiGeometry, Placemark, Point, Polygon};

// TODO: Use this in reader implementation
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum KmlVersion {
    Unknown,
    V22,
    V23,
}

impl Default for KmlVersion {
    fn default() -> KmlVersion {
        KmlVersion::Unknown
    }
}

impl FromStr for KmlVersion {
    type Err = Error;

    // TODO: Support different Google Earth implementations? Only check end?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http://www.opengis.net/kml/2.2" => Ok(Self::V22),
            "http://www.opengis.net/kml/2.3" => Ok(Self::V23),
            v => Err(Error::InvalidKmlVersion(v.to_string())),
        }
    }
}

#[derive(Default, Debug)]
pub struct KmlDocument<T: Float = f64> {
    pub version: KmlVersion,
    pub elements: Vec<Kml<T>>,
}

// Should represent all potential top-level types, maybe all generally?
#[derive(Debug)]
#[non_exhaustive]
pub enum Kml<T: Float = f64> {
    KmlDocument(KmlDocument<T>),
    Point(Point<T>),
    LineString(LineString<T>),
    LinearRing(LinearRing<T>),
    Polygon(Polygon<T>),
    MultiGeometry(MultiGeometry<T>),
    Placemark(Placemark<T>),
    Document { elements: Vec<Kml<T>> },
    Folder { elements: Vec<Kml<T>> },
    Element(Element),
}
