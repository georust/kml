//! Module for converting KML geometry elements to and from `geo-types` primitives
//!
//! # Example
//!
//! ```
//! use kml::types::Coord;
//! use geo_types;
//!
//! let kml_coord = Coord::new(1.0, 1.0, None);
//! let geo_coord = geo_types::Coordinate::from(kml_coord);
//! let kml_coord: Coord = Coord::from(geo_coord);
//! ```
use std::convert::TryFrom;

use crate::errors::Error;
use crate::types::{
    Coord, CoordType, Geometry, Kml, LineString, LinearRing, MultiGeometry, Point, Polygon,
};

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Coordinate<T>> for Coord<T>
where
    T: CoordType,
{
    fn from(val: geo_types::Coordinate<T>) -> Coord<T> {
        Coord::from((val.x, val.y))
    }
}

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<Coord<T>> for geo_types::Coordinate<T>
where
    T: CoordType,
{
    fn from(val: Coord<T>) -> geo_types::Coordinate<T> {
        geo_types::Coordinate::from((val.x, val.y))
    }
}

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Point<T>> for Point<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::Point<T>) -> Point<T> {
        Point::from(Coord::from(val.0))
    }
}

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<Point<T>> for geo_types::Point<T>
where
    T: CoordType,
{
    fn from(val: Point<T>) -> geo_types::Point<T> {
        geo_types::Point::from(geo_types::Coordinate::from(val.coord))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Line<T>> for LineString<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::Line<T>) -> LineString<T> {
        LineString::from(vec![Coord::from(val.start), Coord::from(val.end)])
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::LineString<T>> for LineString<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::LineString<T>) -> LineString<T> {
        LineString::from(
            val.0
                .into_iter()
                .map(Coord::from)
                .collect::<Vec<Coord<T>>>(),
        )
    }
}

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<LineString<T>> for geo_types::LineString<T>
where
    T: CoordType,
{
    fn from(val: LineString<T>) -> geo_types::LineString<T> {
        geo_types::LineString(
            val.coords
                .into_iter()
                .map(geo_types::Coordinate::from)
                .collect(),
        )
    }
}

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::LineString<T>> for LinearRing<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::LineString<T>) -> LinearRing<T> {
        LinearRing::from(
            val.0
                .into_iter()
                .map(Coord::from)
                .collect::<Vec<Coord<T>>>(),
        )
    }
}

#[allow(deprecated)]
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<LinearRing<T>> for geo_types::LineString<T>
where
    T: CoordType,
{
    fn from(val: LinearRing<T>) -> geo_types::LineString<T> {
        geo_types::LineString(
            val.coords
                .into_iter()
                .map(geo_types::Coordinate::from)
                .collect(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Polygon<T>> for Polygon<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::Polygon<T>) -> Polygon<T> {
        let (outer, inner) = val.into_inner();
        Polygon::new(
            LinearRing::from(outer),
            inner
                .into_iter()
                .map(LinearRing::from)
                .collect::<Vec<LinearRing<T>>>(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Rect<T>> for Polygon<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::Rect<T>) -> Polygon<T> {
        Polygon::from(val.to_polygon())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Triangle<T>> for Polygon<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::Triangle<T>) -> Polygon<T> {
        Polygon::from(val.to_polygon())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<Polygon<T>> for geo_types::Polygon<T>
where
    T: CoordType,
{
    fn from(val: Polygon<T>) -> geo_types::Polygon<T> {
        geo_types::Polygon::new(
            geo_types::LineString::from(val.outer),
            val.inner
                .into_iter()
                .map(geo_types::LineString::from)
                .collect::<Vec<geo_types::LineString<T>>>(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::MultiPoint<T>> for MultiGeometry<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::MultiPoint<T>) -> MultiGeometry<T> {
        MultiGeometry::new(
            val.into_iter()
                .map(|p| Geometry::Point(Point::from(p)))
                .collect::<Vec<Geometry<T>>>(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::MultiLineString<T>> for MultiGeometry<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::MultiLineString<T>) -> MultiGeometry<T> {
        MultiGeometry::new(
            val.into_iter()
                .map(|l| Geometry::LineString(LineString::from(l)))
                .collect::<Vec<Geometry<T>>>(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::MultiPolygon<T>> for MultiGeometry<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::MultiPolygon<T>) -> MultiGeometry<T> {
        MultiGeometry::new(
            val.into_iter()
                .map(|p| Geometry::Polygon(Polygon::from(p)))
                .collect::<Vec<Geometry<T>>>(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::GeometryCollection<T>> for MultiGeometry<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::GeometryCollection<T>) -> MultiGeometry<T> {
        MultiGeometry::new(
            val.into_iter()
                .map(Geometry::from)
                .collect::<Vec<Geometry<T>>>(),
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<MultiGeometry<T>> for geo_types::GeometryCollection<T>
where
    T: CoordType,
{
    type Error = Error;

    fn try_from(val: MultiGeometry<T>) -> Result<geo_types::GeometryCollection<T>, Self::Error> {
        Ok(geo_types::GeometryCollection(
            val.geometries
                .into_iter()
                .map(geo_types::Geometry::try_from)
                .collect::<Result<Vec<geo_types::Geometry<T>>, _>>()?,
        ))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<geo_types::Geometry<T>> for Geometry<T>
where
    T: CoordType + Default,
{
    fn from(val: geo_types::Geometry<T>) -> Geometry<T> {
        match val {
            geo_types::Geometry::Point(p) => Geometry::Point(Point::from(p)),
            geo_types::Geometry::Line(l) => Geometry::LineString(LineString::from(l)),
            geo_types::Geometry::LineString(l) => Geometry::LineString(LineString::from(l)),
            geo_types::Geometry::Polygon(p) => Geometry::Polygon(Polygon::from(p)),
            geo_types::Geometry::MultiPoint(p) => Geometry::MultiGeometry(MultiGeometry::from(p)),
            geo_types::Geometry::MultiLineString(l) => {
                Geometry::MultiGeometry(MultiGeometry::from(l))
            }
            geo_types::Geometry::MultiPolygon(p) => Geometry::MultiGeometry(MultiGeometry::from(p)),
            geo_types::Geometry::GeometryCollection(g) => {
                Geometry::MultiGeometry(MultiGeometry::from(g))
            }
            geo_types::Geometry::Rect(r) => Geometry::Polygon(Polygon::from(r)),
            geo_types::Geometry::Triangle(t) => Geometry::Polygon(Polygon::from(t)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<Geometry<T>> for geo_types::Geometry<T>
where
    T: CoordType,
{
    type Error = Error;

    fn try_from(val: Geometry<T>) -> Result<geo_types::Geometry<T>, Self::Error> {
        match val {
            Geometry::Point(p) => Ok(geo_types::Geometry::Point(geo_types::Point::from(p))),
            Geometry::LineString(l) => Ok(geo_types::Geometry::LineString(
                geo_types::LineString::from(l),
            )),
            Geometry::LinearRing(l) => Ok(geo_types::Geometry::LineString(
                geo_types::LineString::from(l),
            )),
            Geometry::Polygon(p) => Ok(geo_types::Geometry::Polygon(geo_types::Polygon::from(p))),
            Geometry::MultiGeometry(g) => Ok(geo_types::Geometry::GeometryCollection(
                geo_types::GeometryCollection::try_from(g)?,
            )),
            _ => Err(Error::InvalidGeometry("Can't convert geometry".to_string())),
        }
    }
}

fn process_kml<T>(k: Kml<T>) -> Result<Vec<geo_types::Geometry<T>>, Error>
where
    T: CoordType,
{
    match k {
        Kml::KmlDocument(d) => Ok(d
            .elements
            .into_iter()
            .flat_map(process_kml)
            .flatten()
            .collect()),
        Kml::Point(p) => Ok(vec![
            geo_types::Geometry::Point(geo_types::Point::from(p));
            1
        ]),
        Kml::LineString(l) => Ok(vec![
            geo_types::Geometry::LineString(
                geo_types::LineString::from(l),
            );
            1
        ]),
        Kml::LinearRing(l) => Ok(vec![
            geo_types::Geometry::LineString(
                geo_types::LineString::from(l),
            );
            1
        ]),
        Kml::Polygon(p) => Ok(vec![
            geo_types::Geometry::Polygon(geo_types::Polygon::from(
                p
            ));
            1
        ]),
        Kml::MultiGeometry(g) => Ok(geo_types::GeometryCollection::try_from(g)?.0),
        Kml::Placemark(p) => Ok(if let Some(g) = p.geometry {
            vec![geo_types::Geometry::try_from(g)?; 1]
        } else {
            vec![]
        }),
        Kml::Document { elements, .. } => Ok(elements
            .into_iter()
            .flat_map(process_kml)
            .flatten()
            .collect()),
        Kml::Folder { elements, .. } => Ok(elements
            .into_iter()
            .flat_map(process_kml)
            .flatten()
            .collect()),
        _ => Ok(vec![]),
    }
}

/// A shortcut for producing `geo-types` [GeometryCollection](../geo_types/struct.GeometryCollection.html)
/// from valid KML input.
///
/// # Example
///
/// ```
/// use geo_types::GeometryCollection;
/// use kml::{quick_collection, Kml};
///
/// let kml_str = r#"
/// <Folder>
///   <Point>
///     <coordinates>1,1,1</coordinates>
///     <altitudeMode>relativeToGround</altitudeMode>
///   </Point>
///   <LineString>
///     <coordinates>1,1 2,1 3,1</coordinates>
///     <altitudeMode>relativeToGround</altitudeMode>
///   </LineString>
/// </Folder>"#;
/// let k: Kml<f64> = kml_str.parse().unwrap();
/// // Turn the KML string into a geo_types GeometryCollection
/// let mut collection: GeometryCollection<f64> = quick_collection(k).unwrap();
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
pub fn quick_collection<T>(k: Kml<T>) -> Result<geo_types::GeometryCollection<T>, Error>
where
    T: CoordType,
{
    Ok(geo_types::GeometryCollection(process_kml(k)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KmlDocument;
    use std::collections::HashMap;

    #[test]
    fn test_quick_collection() {
        let k = KmlDocument {
            elements: vec![
                Kml::Point(Point::from(Coord::from((1., 1.)))),
                Kml::Folder {
                    attrs: HashMap::new(),
                    elements: vec![
                        Kml::LineString(LineString::from(vec![
                            Coord::from((1., 1.)),
                            Coord::from((2., 2.)),
                        ])),
                        Kml::Point(Point::from(Coord::from((3., 3.)))),
                    ],
                },
            ],
            ..Default::default()
        };

        let gc = geo_types::GeometryCollection(vec![
            geo_types::Geometry::Point(geo_types::Point::from((1., 1.))),
            geo_types::Geometry::LineString(geo_types::LineString::from(vec![(1., 1.), (2., 2.)])),
            geo_types::Geometry::Point(geo_types::Point::from((3., 3.))),
        ]);
        assert_eq!(quick_collection(Kml::KmlDocument(k)).unwrap(), gc);
    }
}
