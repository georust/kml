use crate::types;

use num_traits::Float;
use std::convert::TryFrom;

// TODO: Should these be From instead of TryFrom?
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
        Ok(create_geo_line_string(&val.coords))
    }
}

impl<T> TryFrom<types::LinearRing> for geo_types::LineString<T>
where
    T: Float,
{
    type Error = ();

    fn try_from(val: types::LinearRing) -> Result<geo_types::LineString<T>, Self::Error> {
        Ok(create_geo_line_string(&val.coords))
    }
}

impl<T> TryFrom<types::Polygon> for geo_types::Polygon<T>
where
    T: Float,
{
    type Error = ();

    fn try_from(val: types::Polygon) -> Result<geo_types::Polygon<T>, Self::Error> {
        Ok(geo_types::Polygon::new(
            create_geo_line_string(&val.outer.coords),
            val.inner
                .iter()
                .map(|l| create_geo_line_string(&l.coords))
                .collect::<Vec<geo_types::LineString<T>>>(),
        ))
    }
}

// impl<T> TryFrom<types::Geometry> for geo_types::Geometry<T>
// where
//     T: Float,
// {
//     type Error = ();

//     fn try_from(val: types::Geometry) -> Result<geo_types::Geometry<T>, Self::Error> {
//         match val {
//             types::Geometry::Point(p) => Ok(geo_types::Geometry::<T>::from(
//                 geo_types::Point::<T>::try_from(p)?,
//             )),
//             _ => Self::Error(()),
//         }
//     }
// }

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

fn create_geo_line_string<T>(coords: &[types::Coord]) -> geo_types::LineString<T>
where
    T: Float,
{
    geo_types::LineString(coords.iter().map(create_geo_coordinate).collect())
}
