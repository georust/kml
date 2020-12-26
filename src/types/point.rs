use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point<T: Float = f64> {
    pub coord: Coord<T>,
    pub extrude: bool,
    pub altitude_mode: AltitudeMode,
}

impl<T> From<Coord<T>> for Point<T>
where
    T: Float + Default,
{
    fn from(coord: Coord<T>) -> Self {
        Point {
            coord,
            ..Default::default()
        }
    }
}
