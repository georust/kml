use std::collections::HashMap;

use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::linear_ring::LinearRing;

// TODO: Are all geometry fields nullable?
// TODO: Does linear ring only have tessellate or polygon too?
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polygon<T: Float = f64> {
    pub outer: LinearRing<T>,
    pub inner: Vec<LinearRing<T>>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}
