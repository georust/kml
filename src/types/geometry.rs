use crate::types::coord::CoordType;
use crate::types::element::Element;
use crate::types::line_string::LineString;
use crate::types::linear_ring::LinearRing;
use crate::types::multi_geometry::MultiGeometry;
use crate::types::point::Point;
use crate::types::polygon::Polygon;

/// Represents elements in `kml:AbstractGeometryGroup`, [10.1](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#432)
/// in the KML specification
///
/// `kml:Model` is currently represented by a placeholder element
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry<T: CoordType = f64> {
    Point(Point<T>),
    LineString(LineString<T>),
    LinearRing(LinearRing<T>),
    Polygon(Polygon<T>),
    MultiGeometry(MultiGeometry<T>),
    Element(Element), // Currently just a stand-in for Model
}
