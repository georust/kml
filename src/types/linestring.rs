use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineString {
    pub coords: Vec<Coord>,
    pub extrude: bool,
    pub tesselate: bool,
    pub altitude_mode: AltitudeMode,
}
