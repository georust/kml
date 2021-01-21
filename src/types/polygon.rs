use std::collections::HashMap;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::CoordType;
use crate::types::linear_ring::LinearRing;

/// `kml:Polygon`, [10.8](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#505) in the KML
/// specification
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polygon<T: CoordType = f64> {
    pub outer: LinearRing<T>,
    pub inner: Vec<LinearRing<T>>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}

impl<T> Polygon<T>
where
    T: CoordType + Default,
{
    pub fn new(outer: LinearRing<T>, inner: Vec<LinearRing<T>>) -> Self {
        Polygon {
            outer,
            inner,
            ..Default::default()
        }
    }
}
