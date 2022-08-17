use std::collections::HashMap;

use crate::types::coord::CoordType;

/// `kml:Location`, [10.10](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#542) in the KML
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Location<T: CoordType = f64> {
    pub latitude: T,
    pub longitude: T,
    pub altitude: T,
    pub attrs: HashMap<String, String>,
}

impl<T> Location<T>
where
    T: CoordType + Default,
{
    pub fn new(latitude: T, longitude: T, altitude: T) -> Self {
        Location {
            latitude,
            longitude,
            altitude,
            ..Default::default()
        }
    }
}
