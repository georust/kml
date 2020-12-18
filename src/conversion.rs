use crate::types;

use num_traits::Float;
use std::convert::TryFrom;

impl<T> TryFrom<types::Point> for geo_types::Point<T>
where
    T: Float,
{
    type Error = ();

    fn try_from(val: types::Point) -> Result<geo_types::Point<T>, Self::Error> {
        Ok(create_geo_point(&val))
    }
}

impl<T> TryFrom<types::LineString> for geo_types::LineString<T>
where
    T: Float,
{
    type Error = ();

    fn try_from(val: types::LineString) -> Result<geo_types::LineString<T>, Self::Error> {
        Ok(create_geo_line_string(&val))
    }
}

fn create_geo_coordinate<T>(coord: &types::Coord) -> geo_types::Coordinate<T>
where
    T: Float,
{
    // TODO: Should this call unwrap or throw specific error?
    geo_types::Coordinate {
        x: T::from(coord.x).unwrap(),
        y: T::from(coord.y).unwrap(),
    }
}

fn create_geo_point<T>(point: &types::Point) -> geo_types::Point<T>
where
    T: Float,
{
    geo_types::Point::new(
        T::from(point.coord.x).unwrap(),
        T::from(point.coord.y).unwrap(),
    )
}

fn create_geo_line_string<T>(line_string: &types::LineString) -> geo_types::LineString<T>
where
    T: Float,
{
    geo_types::LineString(
        line_string
            .coords
            .iter()
            .map(create_geo_coordinate)
            .collect(),
    )
}
