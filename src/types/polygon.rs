use crate::types::altitude_mode::AltitudeMode;
use crate::types::linearring::LinearRing;

pub struct Polygon {
    pub outer: LinearRing,
    pub inner: Vec<LinearRing>,
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
}
