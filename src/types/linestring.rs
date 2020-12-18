use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Clone, Default)]
pub struct LineString {
    pub coords: Vec<Coord>,
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
}
