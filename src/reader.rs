use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::BufRead;
use std::marker::PhantomData;
use std::str;
use std::str::FromStr;

use num_traits::Float;
use quick_xml::events::{BytesStart, Event};

use crate::errors::Error;
use crate::types::geom_props::GeomProps;
use crate::types::{
    self, coords_from_str, Coord, Element, Geometry, Kml, KmlDocument, KmlVersion, LineString,
    LinearRing, MultiGeometry, Placemark, Point, Polygon,
};

pub struct KmlReader<B: BufRead, T: Float + FromStr + Default + Debug = f64> {
    reader: quick_xml::Reader<B>,
    buf: Vec<u8>,
    version: KmlVersion, // TODO: How to incorporate this so it can be set before parsing?
    _phantom: PhantomData<T>,
}

impl<'a, T> KmlReader<&'a [u8], T>
where
    T: Float + FromStr + Default + Debug,
{
    pub fn from_string(s: &str) -> KmlReader<&[u8], T> {
        KmlReader::<&[u8], T>::from_xml_reader(quick_xml::Reader::<&[u8]>::from_str(s))
    }
}

impl<B: BufRead, T> KmlReader<B, T>
where
    T: Float + FromStr + Default + Debug,
{
    fn from_xml_reader(mut reader: quick_xml::Reader<B>) -> KmlReader<B, T> {
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        KmlReader {
            reader,
            buf: Vec::new(),
            version: KmlVersion::Unknown,
            _phantom: PhantomData,
        }
    }

    pub fn parse(&mut self) -> Result<Kml<T>, Error> {
        let mut result = self.parse_elements()?;
        // Converts multiple items at the same level to KmlDocument
        match result.len().cmp(&1) {
            Ordering::Greater => Ok(Kml::KmlDocument(KmlDocument {
                elements: result,
                ..Default::default()
            })),
            Ordering::Equal => Ok(result.remove(0)),
            Ordering::Less => Err(Error::NoElements),
        }
    }

    fn parse_elements(&mut self) -> Result<Vec<Kml<T>>, Error> {
        let mut elements: Vec<Kml<T>> = Vec::new();
        loop {
            let e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    match e.local_name() {
                        b"kml" => elements.push(Kml::KmlDocument(self.parse_kml_document()?)),
                        b"Point" => elements.push(Kml::Point(self.parse_point()?)),
                        b"LineString" => elements.push(Kml::LineString(self.parse_line_string()?)),
                        b"LinearRing" => elements.push(Kml::LinearRing(self.parse_linear_ring()?)),
                        b"Polygon" => elements.push(Kml::Polygon(self.parse_polygon()?)),
                        b"MultiGeometry" => {
                            elements.push(Kml::MultiGeometry(self.parse_multi_geometry()?))
                        }
                        b"Placemark" => elements.push(Kml::Placemark(self.parse_placemark()?)),
                        b"Document" => elements.push(Kml::Document {
                            elements: self.parse_elements()?,
                        }),
                        b"Folder" => elements.push(Kml::Folder {
                            elements: self.parse_elements()?,
                        }),
                        _ => {
                            // Need to call to_owned() here to avoid duplicate multiple borrow E0499
                            let start = e.to_owned();
                            elements.push(Kml::Element(self.parse_element(&start)?));
                        }
                    };
                }
                Event::Decl(_)
                | Event::CData(_)
                | Event::Empty(_)
                | Event::Text(_)
                | Event::End(_) => {}
                Event::Eof => break,
                _ => return Err(Error::InvalidInput),
            };
        }

        Ok(elements)
    }

    fn parse_kml_document(&mut self) -> Result<KmlDocument<T>, Error> {
        // TODO: Should parse version, change version based on NS
        Ok(KmlDocument {
            elements: self.parse_elements()?,
            ..Default::default()
        })
    }

    fn parse_point(&mut self) -> Result<Point<T>, Error> {
        let mut props = self.parse_geom_props(b"Point")?;
        Ok(Point {
            coord: props.coords.remove(0),
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
        })
    }

    fn parse_line_string(&mut self) -> Result<LineString<T>, Error> {
        let props = self.parse_geom_props(b"LineString")?;
        Ok(LineString {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
        })
    }

    fn parse_linear_ring(&mut self) -> Result<LinearRing<T>, Error> {
        let props = self.parse_geom_props(b"LinearRing")?;
        Ok(LinearRing {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
        })
    }

    fn parse_polygon(&mut self) -> Result<Polygon<T>, Error> {
        let mut outer: LinearRing<T> = LinearRing::default();
        let mut inner: Vec<LinearRing<T>> = Vec::new();
        let mut altitude_mode = types::AltitudeMode::default();
        let mut extrude = false;
        let mut tessellate = false;

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"outerBoundaryIs" => {
                        // TODO: remove because required?
                        outer = self.parse_boundary(b"outerBoundaryIs")?.remove(0)
                    }
                    b"innerBoundaryIs" => inner = self.parse_boundary(b"innerBoundaryIs")?,
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.parse_str()?).unwrap()
                    }
                    b"extrude" => extrude = self.parse_str()? == "1",
                    b"tessellate" => tessellate = self.parse_str()? == "1",
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Polygon" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(Polygon {
            outer,
            inner,
            altitude_mode,
            extrude,
            tessellate,
        })
    }

    fn parse_multi_geometry(&mut self) -> Result<MultiGeometry<T>, Error> {
        let mut geometries: Vec<Geometry<T>> = Vec::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"Point" => geometries.push(Geometry::Point(self.parse_point()?)),
                    b"LineString" => {
                        geometries.push(Geometry::LineString(self.parse_line_string()?))
                    }
                    b"LinearRing" => {
                        geometries.push(Geometry::LinearRing(self.parse_linear_ring()?))
                    }
                    b"Polygon" => geometries.push(Geometry::Polygon(self.parse_polygon()?)),
                    // TODO: Can multi_geometry be nested?
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"MultiGeometry" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(MultiGeometry(geometries))
    }

    fn parse_placemark(&mut self) -> Result<Placemark<T>, Error> {
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut geometry: Option<Geometry<T>> = None;

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"name" => name = Some(self.parse_str()?),
                    b"description" => description = Some(self.parse_str()?),
                    b"Point" => geometry = Some(Geometry::Point(self.parse_point()?)),
                    b"LineString" => {
                        geometry = Some(Geometry::LineString(self.parse_line_string()?))
                    }
                    b"LinearRing" => {
                        geometry = Some(Geometry::LinearRing(self.parse_linear_ring()?))
                    }
                    b"Polygon" => geometry = Some(Geometry::Polygon(self.parse_polygon()?)),
                    b"MultiGeometry" => {
                        geometry = Some(Geometry::MultiGeometry(self.parse_multi_geometry()?))
                    }
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Placemark" {
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(Placemark {
            name,
            description,
            geometry,
        })
    }

    fn parse_element(&mut self, start: &BytesStart) -> Result<Element, Error> {
        let mut element = Element::default();
        let tag = start.local_name();
        element.name = str::from_utf8(tag).unwrap().to_string();
        element.attrs = self.parse_attrs(start)?;
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    let e_start = e.to_owned();
                    element.children.push(self.parse_element(&e_start)?);
                }
                Event::Text(ref mut e) => {
                    element.content = Some(e.unescape_and_decode(&self.reader).expect("Error"))
                }
                Event::End(ref mut e) => {
                    if e.local_name() == tag {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(element)
    }

    fn parse_boundary(&mut self, end_tag: &[u8]) -> Result<Vec<LinearRing<T>>, Error> {
        let mut boundary: Vec<LinearRing<T>> = Vec::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    if e.local_name() == b"LinearRing" {
                        boundary.push(self.parse_linear_ring()?);
                    }
                }
                Event::End(ref mut e) => {
                    if e.local_name() == end_tag {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(boundary)
    }

    fn parse_geom_props(&mut self, end_tag: &[u8]) -> Result<GeomProps<T>, Error> {
        let mut coords: Vec<Coord<T>> = Vec::new();
        let mut altitude_mode = types::AltitudeMode::default();
        let mut extrude = false;
        let mut tessellate = false;

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"coordinates" => {
                        coords = coords_from_str(&self.parse_str()?).unwrap();
                    }
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.parse_str()?).unwrap()
                    }
                    b"extrude" => extrude = self.parse_str()? == "1",
                    b"tessellate" => tessellate = self.parse_str()? == "1",
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == end_tag {
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(GeomProps {
            coords,
            altitude_mode,
            extrude,
            tessellate,
        })
    }

    fn parse_str(&mut self) -> Result<String, Error> {
        let e = self.reader.read_event(&mut self.buf)?;
        match e {
            Event::Text(e) | Event::CData(e) => {
                Ok(e.unescape_and_decode(&self.reader).expect("Error"))
            }
            e => Err(Error::InvalidXmlEvent(format!("{:?}", e))), // TODO: Not sure if right approach
        }
    }

    fn parse_attrs(&self, start: &BytesStart) -> Result<HashMap<String, String>, Error> {
        let attrs = start
            .attributes()
            .filter_map(Result::ok)
            .map(|a| {
                (
                    str::from_utf8(a.key).unwrap().to_string(),
                    str::from_utf8(&a.value).unwrap().to_string(),
                )
            })
            .collect();
        Ok(attrs)
    }
}

impl<T> FromStr for Kml<T>
where
    T: Float + FromStr + Default + Debug,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KmlReader::<&[u8], T>::from_string(s).parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let kml_str = "<Point><coordinates>1,1,1</coordinates><altitudeMode>relativeToGround</altitudeMode></Point>";
        let p: Kml = kml_str.parse().unwrap();
        assert!(matches!(p, Kml::Point(_)));
        match p {
            Kml::Point(p) => assert_eq!(
                p,
                Point {
                    coord: Coord {
                        x: 1.,
                        y: 1.,
                        z: Some(1.)
                    },
                    altitude_mode: types::AltitudeMode::RelativeToGround,
                    extrude: false,
                }
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_line_string() {
        let kml_str = r#"<LineString>
            <coordinates>1,1 2,1 3,1</coordinates>
            <altitudeMode>relativeToGround</altitudeMode>
        </LineString>"#;
        let l: Kml = kml_str.parse().unwrap();
        assert!(matches!(l, Kml::LineString(_)));
        match l {
            Kml::LineString(l) => assert_eq!(
                l,
                LineString {
                    coords: vec![
                        Coord {
                            x: 1.,
                            y: 1.,
                            z: None
                        },
                        Coord {
                            x: 2.,
                            y: 1.,
                            z: None
                        },
                        Coord {
                            x: 3.,
                            y: 1.,
                            z: None
                        }
                    ],
                    altitude_mode: types::AltitudeMode::RelativeToGround,
                    extrude: false,
                    tessellate: false
                }
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_polygon() {
        let poly_str = r#"<Polygon>
        <outerBoundaryIs>
          <LinearRing>
            <tessellate>1</tessellate>
            <coordinates>
              -1,2,0
              -1.5,3,0
              -1.5,2,0
              -1,2,0
            </coordinates>
          </LinearRing>
        </outerBoundaryIs>
      </Polygon>"#;
        let mut r = KmlReader::from_string(poly_str);

        let p: Kml = r.parse().unwrap();
        assert!(matches!(p, Kml::Polygon(_)));
        match p {
            Kml::Polygon(p) => assert_eq!(
                p,
                Polygon {
                    outer: LinearRing {
                        coords: vec![
                            Coord {
                                x: -1.,
                                y: 2.,
                                z: Some(0.)
                            },
                            Coord {
                                x: -1.5,
                                y: 3.,
                                z: Some(0.)
                            },
                            Coord {
                                x: -1.5,
                                y: 2.,
                                z: Some(0.)
                            },
                            Coord {
                                x: -1.,
                                y: 2.,
                                z: Some(0.)
                            },
                        ],
                        extrude: false,
                        tessellate: true,
                        altitude_mode: types::AltitudeMode::ClampToGround,
                    },
                    inner: vec![],
                    extrude: false,
                    tessellate: false,
                    altitude_mode: types::AltitudeMode::ClampToGround
                }
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_kml_document_default() {
        let kml_str ="<Point><coordinates>1,1,1</coordinates></Point><LineString><coordinates>1,1 2,1</coordinates></LineString>";
        let d: Kml = kml_str.parse().unwrap();

        assert!(matches!(d, Kml::KmlDocument(_)));
        let doc: Option<KmlDocument> = match d {
            Kml::KmlDocument(d) => Some(d),
            _ => None,
        };

        assert!(doc.unwrap().elements.iter().all(|e| match e {
            Kml::Point(p) =>
                *p == Point {
                    coord: Coord {
                        x: 1.,
                        y: 1.,
                        z: Some(1.)
                    },
                    altitude_mode: types::AltitudeMode::ClampToGround,
                    extrude: false,
                },
            Kml::LineString(l) =>
                *l == LineString {
                    coords: vec![
                        Coord {
                            x: 1.,
                            y: 1.,
                            z: None
                        },
                        Coord {
                            x: 2.,
                            y: 1.,
                            z: None
                        },
                    ],
                    altitude_mode: types::AltitudeMode::ClampToGround,
                    extrude: false,
                    tessellate: false
                },
            _ => false,
        }))
    }

    #[test]
    fn test_parse() {
        let kml_str = include_str!("../fixtures/sample.kml");

        assert!(matches!(
            Kml::<f64>::from_str(kml_str).unwrap(),
            Kml::KmlDocument(_)
        ))
    }
}
