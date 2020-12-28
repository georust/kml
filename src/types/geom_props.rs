use std::fmt::Debug;
use std::str::FromStr;

use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

// TODO: Should this be an attribute of geometries? Only complication is Point doesn't include
// tessellate, not sure how to represent that
pub(crate) struct GeomProps<T: Float + FromStr + Default + Debug = f64> {
    pub coords: Vec<Coord<T>>,
    pub altitude_mode: AltitudeMode,
    pub extrude: bool,
    pub tessellate: bool,
}
