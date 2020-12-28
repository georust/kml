use num_traits::Float;

use crate::types::element::Element;
use crate::types::line_string::LineString;
use crate::types::linear_ring::LinearRing;
use crate::types::multi_geometry::MultiGeometry;
use crate::types::point::Point;
use crate::types::polygon::Polygon;

/// AbstractGeometryGroup
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry<T: Float = f64> {
    Point(Point<T>),
    LineString(LineString<T>),
    LinearRing(LinearRing<T>),
    Polygon(Polygon<T>),
    MultiGeometry(MultiGeometry<T>),
    Element(Element), // Currently just a stand-in for Model
}
