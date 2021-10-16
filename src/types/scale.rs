use std::collections::HashMap;

use crate::types::coord::CoordType;
use num_traits::One;

/// `kml:Scale`, [10.12](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#575) in the KML
#[derive(Clone, Debug, PartialEq)]
pub struct Scale<T: CoordType = f64> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub attrs: HashMap<String, String>,
}

impl<T> Scale<T>
where
  T: CoordType,
{
  pub fn new(x: T, y: T, z: T) -> Self {
    Scale {
      x,
      y,
      z,
      attrs: HashMap::new(),
    }
  }
}

impl Default for Scale {
  fn default() -> Scale {
    Scale {
      x: One::one(),
      y: One::one(),
      z: One::one(),
      attrs: HashMap::new(),
    }
  }
}
