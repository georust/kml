use crate::types::altitude_mode::AltitudeMode;
use crate::types::linearring::LinearRing;

// TODO: Are all geometry fields nullable?
// TODO: Does linear ring only have tessellate or polygon too?
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polygon {
    pub outer: LinearRing,
    pub inner: Vec<LinearRing>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
}
