use std::collections::HashMap;

use crate::types::coord::CoordType;
use crate::types::geometry::Geometry;

/// `kml:MultiGeometry`, [10.2](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#438) in the
/// KML specification
#[derive(Clone, Default, PartialEq, Debug)]
pub struct MultiGeometry<T: CoordType = f64> {
    pub geometries: Vec<Geometry<T>>,
    pub attrs: HashMap<String, String>,
}

impl<T> MultiGeometry<T>
where
    T: CoordType + Default,
{
    pub fn new(geometries: Vec<Geometry<T>>) -> Self {
        MultiGeometry {
            geometries,
            ..Default::default()
        }
    }
}
