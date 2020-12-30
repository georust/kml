use std::collections::HashMap;

use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::linear_ring::LinearRing;

/// `kml:Polygon`, [10.8](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#505) in the KML
/// specification
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polygon<T: Float = f64> {
    pub outer: LinearRing<T>,
    pub inner: Vec<LinearRing<T>>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}
