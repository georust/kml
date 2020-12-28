use std::fmt::Debug;
use std::str::FromStr;

use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

pub(crate) struct GeomProps<T: Float + FromStr + Default + Debug = f64> {
    pub coords: Vec<Coord<T>>,
    pub altitude_mode: AltitudeMode,
    pub extrude: bool,
    pub tessellate: bool,
}
