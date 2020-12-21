use crate::types::altitude_mode::AltitudeMode;
use crate::types::linearring::LinearRing;

#[derive(Clone, Debug, Default)]
pub struct Polygon {
    pub outer: LinearRing,
    pub inner: Vec<LinearRing>,
    pub extrude: bool,
    pub tesselate: bool,
    pub altitude_mode: AltitudeMode,
}
