use std::num::ParseFloatError;
use std::str::FromStr;

// Coordinates are delimited by any whitespace and items within tuples separated by commas
// TODO: Specific reference in spec
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

impl FromStr for Coord {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(',');
        let x = parts.next().unwrap_or("").parse::<f64>()?;
        let y = parts.next().unwrap_or("").parse::<f64>()?;
        let z = if let Some(z) = parts.next() {
            Some(z.parse::<f64>()?)
        } else {
            None
        };
        Ok(Coord { x, y, z })
    }
}

pub fn coords_from_str(s: &str) -> Result<Vec<Coord>, ParseFloatError> {
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
