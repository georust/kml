use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Point {
    pub coord: Coord, // TODO: Can this be empty and require Option?
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
}

impl Point {
    pub fn from_coord(coord: Coord) -> Point {
        let mut p = Point::default();
        p.coord = coord;
        p
    }
}
