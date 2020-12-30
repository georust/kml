use std::collections::HashMap;

use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

/// Represents `kml:LineString`, [10.7](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#488)
/// in the KML specification
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineString<T: Float = f64> {
    pub coords: Vec<Coord<T>>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}

impl<T> From<Vec<Coord<T>>> for LineString<T>
where
    T: Float + Default,
{
    fn from(coords: Vec<Coord<T>>) -> Self {
        LineString {
            coords,
            ..Default::default()
        }
    }
}
