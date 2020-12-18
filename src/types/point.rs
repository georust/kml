use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Clone, Default)]
pub struct Point {
    pub coord: Coord, // TODO: Can this be empty and require Option?
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
}
