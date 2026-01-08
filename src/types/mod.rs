//! Module containing types for KML elements
mod altitude_mode;
mod coord;

pub use altitude_mode::AltitudeMode;
pub use coord::{coords_from_str, Coord, CoordType};

mod line_string;
mod linear_ring;
mod location;
mod multi_geometry;
mod orientation;
mod point;
mod polygon;
mod scale;
mod track;
mod vec2;

pub use line_string::LineString;
pub use linear_ring::LinearRing;
pub use location::Location;
pub use multi_geometry::MultiGeometry;
pub use orientation::Orientation;
pub use point::Point;
pub use polygon::Polygon;
pub use scale::Scale;
pub use track::Track;
pub use vec2::{Units, Vec2};

mod element;
pub(crate) mod geom_props;
mod placemark;

mod folder;
pub use folder::Folder;

pub use element::Element;
pub use placemark::Placemark;

mod geometry;

pub use geometry::Geometry;

mod link;

pub use link::{Icon as LinkTypeIcon, Link, RefreshMode, ViewRefreshMode};

mod style;

pub use style::{
    BalloonStyle, ColorMode, Icon, IconStyle, LabelStyle, LineStyle, ListStyle, Pair, PolyStyle,
    Style, StyleMap,
};

mod resource_map;

pub use resource_map::ResourceMap;

mod alias;

pub use alias::Alias;

mod data;

pub use data::{Data, SchemaData, SimpleArrayData, SimpleData};

mod kml;

pub use self::kml::{Kml, KmlDocument, KmlVersion};
