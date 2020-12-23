use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinearRing {
    pub coords: Vec<Coord>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
}
