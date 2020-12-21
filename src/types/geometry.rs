use crate::types::linearring::LinearRing;
use crate::types::linestring::LineString;
use crate::types::point::Point;
use crate::types::polygon::Polygon;

// TODO: Should geometry have separate enum or use main Kml?
// Includes everything within AbstractGeometryGroup
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Geometry {
    Point(Point),
    LineString(LineString),
    LinearRing(LinearRing),
    Polygon(Polygon),
    MultiGeometry(Vec<Geometry>),
}
