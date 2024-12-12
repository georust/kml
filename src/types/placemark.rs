use std::collections::HashMap;

use crate::types::coord::CoordType;
use crate::types::element::Element;
use crate::types::geometry::Geometry;

/// `kml:Placemark`, [9.14](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#249) in the KML
/// specification
///
/// Placemark not inside of kml:Update (unused) requires a Geometry according to [ATC-226](https://docs.opengeospatial.org/ts/14-068r2/14-068r2.html#atc-226),
/// but Google's  reference says it's optional [Google Placemark reference](https://developers.google.com/kml/documentation/kmlreference#placemark).
///
/// Currently leaving optional.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Placemark<T: CoordType = f64> {
    pub name: Option<String>,
    pub description: Option<String>,
    pub geometry: Option<Geometry<T>>,
    pub style_url: Option<String>,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Element>,
}
