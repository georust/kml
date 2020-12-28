use num_traits::Float;

use crate::types::geometry::Geometry;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Placemark<T: Float = f64> {
    pub name: Option<String>,
    pub description: Option<String>,
    pub geometry: Option<Geometry<T>>,
    // TODO: Support other elements
}
