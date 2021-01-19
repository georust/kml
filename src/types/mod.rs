mod altitude_mode;
mod coord;

pub use altitude_mode::AltitudeMode;
pub use coord::{coords_from_str, Coord, CoordType};

mod line_string;
mod linear_ring;
mod multi_geometry;
mod point;
mod polygon;

pub use line_string::LineString;
pub use linear_ring::LinearRing;
pub use multi_geometry::MultiGeometry;
pub use point::Point;
pub use polygon::Polygon;

mod element;
pub(crate) mod geom_props;
mod placemark;

pub use element::Element;
pub use placemark::Placemark;

mod geometry;

pub use geometry::Geometry;

mod style;

pub use style::{
    BalloonStyle, ColorMode, Icon, IconStyle, LabelStyle, LineStyle, ListStyle, Pair, PolyStyle,
    Style, StyleMap,
};

mod kml;

pub use self::kml::{Kml, KmlDocument, KmlVersion};
