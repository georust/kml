use crate::types::{CoordType, Kml};
use std::collections::HashMap;

/// `kml:Folder`, [9.13](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#241) in the KML
/// specification
///
/// Partially implemented.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Folder<T: CoordType = f64> {
    pub name: Option<String>,
    pub description: Option<String>,
    pub attrs: HashMap<String, String>,
    pub elements: Vec<Kml<T>>,
}
