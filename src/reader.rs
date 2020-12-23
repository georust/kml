use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;
use std::str;
use std::str::FromStr;

use crate::types::coords_from_str;
use crate::{
    types, Coord, Element, Geometry, Kml, KmlDocument, LineString, LinearRing, Point, Polygon,
};
use quick_xml::events::{BytesStart, Event};

pub struct KmlReader<B: BufRead> {
    reader: quick_xml::Reader<B>,
    buf: Vec<u8>,
}

impl<'a> KmlReader<&'a [u8]> {
    pub fn from_string(s: &str) -> KmlReader<&[u8]> {
        KmlReader::from_xml_reader(quick_xml::Reader::<&[u8]>::from_str(s))
    }
}

impl<B: BufRead> KmlReader<B> {
    fn from_xml_reader(mut reader: quick_xml::Reader<B>) -> KmlReader<B> {
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        KmlReader {
            reader,
            buf: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Kml, quick_xml::Error> {
        let mut result: Vec<Kml> = Vec::new();
        loop {
            let e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    match e.local_name() {
                        b"Point" => result.push(Kml::Point(self.parse_point()?)),
                        b"LineString" => result.push(Kml::LineString(self.parse_line_string()?)),
                        b"LinearRing" => result.push(Kml::LinearRing(self.parse_linear_ring()?)),
                        b"Polygon" => result.push(Kml::Polygon(self.parse_polygon()?)),
                        b"MultiGeometry" => {
                            result.push(Kml::MultiGeometry(self.parse_multi_geometry()?))
                        }
                        _ => {
                            // Need to call to_owned() here to avoid duplicate multiple borrow E0499
                            let start = e.to_owned();
                            result.push(Kml::Element(self.parse_element(&start)?));
                        }
                    };
                }
                Event::Empty(_) | Event::Text(_) | Event::End(_) => {}
                Event::Eof => break,
                _ => return Err(quick_xml::Error::UnexpectedToken("t".to_string())),
            };
        }

        // Converts multiple items at the same level to KmlDocument
        match result.len().cmp(&1) {
            Ordering::Greater => {
                let mut doc = KmlDocument::default();
                doc.elements = result;
                Ok(Kml::KmlDocument(doc))
            }
            Ordering::Equal => Ok(result.remove(0)),
            Ordering::Less => Err(quick_xml::Error::UnexpectedToken("no results".to_string())),
        }
    }

    fn parse_point(&mut self) -> Result<Point, quick_xml::Error> {
        let mut props = self.parse_geom_props(b"Point")?;
        Ok(Point {
            coord: props.coords.remove(0),
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
        })
    }

    fn parse_line_string(&mut self) -> Result<LineString, quick_xml::Error> {
        let props = self.parse_geom_props(b"LineString")?;
        Ok(LineString {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
        })
    }

    fn parse_linear_ring(&mut self) -> Result<LinearRing, quick_xml::Error> {
        let props = self.parse_geom_props(b"LinearRing")?;
        Ok(LinearRing {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
        })
    }

    fn parse_polygon(&mut self) -> Result<Polygon, quick_xml::Error> {
        let mut outer: LinearRing = LinearRing::default();
        let mut inner: Vec<LinearRing> = Vec::new();
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

    fn parse_multi_geometry(&mut self) -> Result<Vec<Geometry>, quick_xml::Error> {
        let mut geometries: Vec<Geometry> = Vec::new();
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
                    // TODO: Can multigeometry be nested?
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
        Ok(geometries)
    }

    fn parse_element(&mut self, start: &BytesStart) -> Result<Element, quick_xml::Error> {
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
                    element.content = e.unescape_and_decode(&self.reader).expect("Error")
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

    fn parse_boundary(&mut self, end_tag: &[u8]) -> Result<Vec<LinearRing>, quick_xml::Error> {
        let mut boundary: Vec<LinearRing> = Vec::new();
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

    fn parse_geom_props(&mut self, end_tag: &[u8]) -> Result<GeomProps, quick_xml::Error> {
        let mut coords: Vec<Coord> = Vec::new();
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

    fn parse_str(&mut self) -> Result<String, quick_xml::Error> {
        let e = self.reader.read_event(&mut self.buf)?;
        match e {
            Event::Text(e) => Ok(e.unescape_and_decode(&self.reader).expect("Error")),
            _ => Err(quick_xml::Error::UnexpectedToken("f".to_string())),
        }
    }

    fn parse_attrs(&self, start: &BytesStart) -> Result<HashMap<String, String>, quick_xml::Error> {
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

// TODO: Might have to include everything here, all possible variables like tessellate
struct GeomProps {
    coords: Vec<Coord>,
    altitude_mode: types::AltitudeMode,
    extrude: bool,
    tessellate: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let mut r = KmlReader::from_string("<Point><coordinates>1,1,1</coordinates><altitudeMode>relativeToGround</altitudeMode></Point>");
        let p: Kml = r.parse().unwrap();
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
        let mut r = KmlReader::from_string(
            "<LineString><coordinates>1,1 2,1 3,1</coordinates><altitudeMode>relativeToGround</altitudeMode></LineString>",
        );
        let l: Kml = r.parse().unwrap();
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
        let mut r = KmlReader::from_string(
            "<Point><coordinates>1,1,1</coordinates></Point><LineString><coordinates>1,1 2,1</coordinates></LineString>",
        );
        let d: Kml = r.parse().unwrap();

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
}
