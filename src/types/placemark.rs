use std::collections::HashMap;

use num_traits::Float;

use crate::types::element::Element;
use crate::types::geometry::Geometry;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Placemark<T: Float = f64> {
    pub name: Option<String>,
    pub description: Option<String>,
    pub geometry: Option<Geometry<T>>,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Element>,
}
