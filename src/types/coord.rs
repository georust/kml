use std::fmt::{self, Debug};
use std::str::FromStr;

use num_traits::Float;

use crate::errors::Error;

/// KML coordinates described by `kml:coordinatesType`, [16.10](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#1212)
/// in the KML specification
///
/// Coordinates are tuples with the third Z value for altitude being optional. Coordinate tuples are separated by any whitespace character
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Coord<T: Float = f64> {
    pub x: T,
    pub y: T,
    pub z: Option<T>,
}

impl<T> Coord<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: Option<T>) -> Self {
        Coord { x, y, z }
    }
}

impl<T> From<(T, T)> for Coord<T>
where
    T: Float,
{
    fn from(coord: (T, T)) -> Self {
        Coord::new(coord.0, coord.1, None)
    }
}

impl<T> From<[T; 2]> for Coord<T>
where
    T: Float,
{
    fn from(coord: [T; 2]) -> Self {
        Coord::new(coord[0], coord[1], None)
    }
}

impl<T> From<(T, T, Option<T>)> for Coord<T>
where
    T: Float,
{
    fn from(coord: (T, T, Option<T>)) -> Self {
        Coord::new(coord.0, coord.1, coord.2)
    }
}

impl<T> From<(T, T, T)> for Coord<T>
where
    T: Float,
{
    fn from(coord: (T, T, T)) -> Self {
        Coord::new(coord.0, coord.1, Some(coord.2))
    }
}

impl<T> From<[T; 3]> for Coord<T>
where
    T: Float,
{
    fn from(coord: [T; 3]) -> Self {
        Coord::new(coord[0], coord[1], Some(coord[2]))
    }
}

impl<T> FromStr for Coord<T>
where
    T: Float + FromStr + Debug,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(',');
        let x_str = parts.next().ok_or(Error::CoordEmpty)?;
        let x: T = x_str
            .parse()
            .map_err(|_| Error::FloatParse(x_str.to_string()))?;
        let y_str = parts.next().ok_or(Error::CoordEmpty)?;
        let y: T = y_str
            .parse()
            .map_err(|_| Error::FloatParse(y_str.to_string()))?;
        let z = if let Some(z) = parts.next() {
            Some(
                z.parse::<T>()
                    .map_err(|_| Error::FloatParse(z.to_string()))?,
            )
        } else {
            None
        };
        Ok(Coord { x, y, z })
    }
}

impl<T> fmt::Display for Coord<T>
where
    T: fmt::Display + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(z) = self.z {
            write!(f, "{},{},{}", self.x, self.y, z)
        } else {
            write!(f, "{},{}", self.x, self.y)
        }
    }
}

/// Utility method for parsing multiple coordinates according to the spec
///
/// # Example
///
/// ```
/// use kml::types::{Coord, coords_from_str};
///
/// let coords_str = "1,1,0\n\n1,2,0  2,2,0";
/// let coords: Vec<Coord> = coords_from_str(coords_str).unwrap();
/// ```
pub fn coords_from_str<T: Float + FromStr + Debug>(s: &str) -> Result<Vec<Coord<T>>, Error> {
    s.split_whitespace().map(Coord::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::{coords_from_str, Coord};
    use std::str::FromStr;

    #[test]
    fn test_coord_from_str() {
        assert_eq!(
            Coord::from_str(" 1.0,2.0,3 ").unwrap(),
            Coord {
                x: 1.,
                y: 2.,
                z: Some(3.)
            }
        );
        assert_eq!(
            Coord::from_str("1,1").unwrap(),
            Coord {
                x: 1.,
                y: 1.,
                z: None
            }
        );
    }

    #[test]
    fn test_coords_from_str() {
        assert_eq!(
            coords_from_str("1,1\n\n 2,2 ").unwrap(),
            vec![
                Coord {
                    x: 1.,
                    y: 1.,
                    z: None
                },
                Coord {
                    x: 2.,
                    y: 2.,
                    z: None
                }
            ]
        )
    }
}
