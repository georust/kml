use std::collections::HashMap;

use crate::types::coord::CoordType;

/// `kml:Orientation`, [10.11](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#558) in the KML
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Orientation<T: CoordType = f64> {
    pub roll: T,
    pub tilt: T,
    pub heading: T,
    pub attrs: HashMap<String, String>,
}

impl<T> Orientation<T>
where
    T: CoordType + Default,
{
    pub fn new(roll: T, tilt: T, heading: T) -> Self {
        Orientation {
            roll,
            tilt,
            heading,
            ..Default::default()
        }
    }
}
