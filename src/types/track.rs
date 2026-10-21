use std::collections::HashMap;

use crate::types::coord::CoordType;
use crate::types::Coord;

/// `kml:Track`, [10.15](https://docs.ogc.org/is/12-007r2/12-007r2.html#611) in the KML
/// specification
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Track<T: CoordType = f64> {
    pub coords: Vec<Coord<T>>,
    pub attrs: HashMap<String, String>,
}

impl<T> Track<T>
where
    T: CoordType + Default,
{
    pub fn new(coords: Vec<Coord<T>>) -> Self {
        Track {
            coords,
            ..Default::default()
        }
    }
}
