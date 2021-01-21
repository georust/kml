use std::collections::HashMap;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::{Coord, CoordType};

/// `kml:Point`, [10.2](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#446) in the KML
/// specification
///
/// Coord is required as of https://docs.opengeospatial.org/ts/14-068r2/14-068r2.html#atc-114
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Point<T: CoordType = f64> {
    pub coord: Coord<T>,
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
    pub attrs: HashMap<String, String>,
}

impl<T> From<Coord<T>> for Point<T>
where
    T: CoordType + Default,
{
    fn from(coord: Coord<T>) -> Self {
        Point {
            coord,
            ..Default::default()
        }
    }
}

impl<T> Point<T>
where
    T: CoordType + Default,
{
    pub fn new(x: T, y: T, z: Option<T>) -> Self {
        Point::from(Coord::new(x, y, z))
    }
}
