use std::collections::HashMap;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::{Coord, CoordType};

/// `kml:LinearRing`, [10.5](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#465) in the
/// KML specification
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinearRing<T: CoordType = f64> {
    pub coords: Vec<Coord<T>>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}

impl<T> From<Vec<Coord<T>>> for LinearRing<T>
where
    T: CoordType + Default,
{
    fn from(coords: Vec<Coord<T>>) -> Self {
        LinearRing {
            coords,
            ..Default::default()
        }
    }
}
