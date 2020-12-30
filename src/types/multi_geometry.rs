use std::collections::HashMap;

use num_traits::Float;

use crate::types::geometry::Geometry;

/// Represents `kml:MultiGeometry`, [10.2](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#438)
/// in the KML specification
#[derive(Clone, Default, PartialEq, Debug)]
pub struct MultiGeometry<T: Float = f64> {
    pub geometries: Vec<Geometry<T>>,
    pub attrs: HashMap<String, String>,
}
