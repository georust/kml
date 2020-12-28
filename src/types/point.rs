use std::collections::HashMap;

use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

/// Point type defined in 10.2
///
/// Coord is required as of https://docs.opengeospatial.org/ts/14-068r2/14-068r2.html#atc-114
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Point<T: Float = f64> {
    pub coord: Coord<T>,
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}

impl<T> From<Coord<T>> for Point<T>
where
    T: Float + Default,
{
    fn from(coord: Coord<T>) -> Self {
        Point {
            coord,
            ..Default::default()
        }
    }
}
