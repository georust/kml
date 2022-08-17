use std::collections::HashMap;
use std::str::FromStr;

use crate::errors::Error;
use crate::types::{
    Alias, BalloonStyle, CoordType, Element, Icon, IconStyle, LabelStyle, LineString, LineStyle,
    LinearRing, Link, LinkTypeIcon, ListStyle, Location, MultiGeometry, Orientation, Pair,
    Placemark, Point, PolyStyle, Polygon, ResourceMap, Scale, Style, StyleMap,
};

/// Enum for representing the KML version being parsed
///
/// According to <http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#7> namespace for 2.3
/// is unchanged since it should be backwards-compatible
#[derive(Clone, Debug, PartialEq, Eq)]
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

// TODO: According to http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#7 namespace for 2.3
// is unchanged since it should be backwards-compatible
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

/// Container for KML root element
#[derive(Clone, Default, PartialEq, Debug)]
pub struct KmlDocument<T: CoordType = f64> {
    pub version: KmlVersion,
    pub attrs: HashMap<String, String>,
    pub elements: Vec<Kml<T>>,
}

/// Enum for representing any KML element
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Kml<T: CoordType = f64> {
    KmlDocument(KmlDocument<T>),
    Scale(Scale<T>),
    Orientation(Orientation<T>),
    Point(Point<T>),
    Location(Location<T>),
    LineString(LineString<T>),
    LinearRing(LinearRing<T>),
    Polygon(Polygon<T>),
    MultiGeometry(MultiGeometry<T>),
    Placemark(Placemark<T>),
    Document {
        attrs: HashMap<String, String>,
        elements: Vec<Kml<T>>,
    },
    Folder {
        attrs: HashMap<String, String>,
        elements: Vec<Kml<T>>,
    },
    Style(Style),
    StyleMap(StyleMap),
    Pair(Pair),
    BalloonStyle(BalloonStyle),
    IconStyle(IconStyle),
    Icon(Icon),
    LabelStyle(LabelStyle),
    LineStyle(LineStyle),
    PolyStyle(PolyStyle),
    ListStyle(ListStyle),
    LinkTypeIcon(LinkTypeIcon),
    Link(Link),
    ResourceMap(ResourceMap),
    Alias(Alias),
    Element(Element),
}
