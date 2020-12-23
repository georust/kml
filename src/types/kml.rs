use crate::types::{Element, Geometry, LineString, LinearRing, Point, Polygon};

// TODO: Only start with abstract group for Feature, Geometry
// TODO: Should Kml contain every tag, with variants specifying more specific elements based on spec?
// Leaning toward second approach

#[derive(Debug, PartialEq)]
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

#[derive(Default, Debug)]
pub struct KmlDocument {
    pub version: KmlVersion,
    pub elements: Vec<Kml>,
}

// TODO: Figure out specific attributes like "name" for Placemark

// TODO: Figure out representing arbitrary attributes, struct for attributes hash and children?

#[derive(Debug)]
#[non_exhaustive]
pub enum Kml {
    KmlDocument(KmlDocument),
    Point(Point),
    LineString(LineString),
    LinearRing(LinearRing),
    Polygon(Polygon),
    MultiGeometry(Vec<Geometry>),
    Placemark(Geometry),
    Document { elements: Vec<Kml> },
    Folder { elements: Vec<Kml> },
    Element(Element),
}
