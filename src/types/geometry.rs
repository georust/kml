use crate::types::linestring::LineString;
use crate::types::point::Point;
use crate::types::polygon::Polygon;
use crate::types::LinearRing::LinearRing;

// TODO: Should geometry have separate enum or use main Kml?
#[non_exhaustive]
pub enum Geometry {
    Point(Point),
    LineString(LineString),
    LinearRing(LinearRing),
    Polygon(Polygon),
    MultiGeometry(Vec<Geometry>),
}
