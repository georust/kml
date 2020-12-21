use std::cmp::Ordering;
use std::io::BufRead;
use std::str;
use std::str::FromStr;

use crate::types::coords_from_str;
use crate::{types, Coord, Kml, KmlDocument, LineString, LinearRing, Point, Polygon};
use quick_xml::events::Event;

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
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    match e.local_name() {
                        b"Point" => result.push(Kml::Point(self.parse_point()?)),
                        b"LineString" => result.push(Kml::LineString(self.parse_line_string()?)),
                        b"LinearRing" => result.push(Kml::LinearRing(self.parse_linear_ring()?)),
                        n => {
                            return Err(quick_xml::Error::UnexpectedToken(
                                str::from_utf8(n).unwrap().to_string(),
                            ))
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
            tesselate: props.tesselate,
        })
    }

    fn parse_linear_ring(&mut self) -> Result<LinearRing, quick_xml::Error> {
        let props = self.parse_geom_props(b"LinearRing")?;
        Ok(LinearRing {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tesselate: props.tesselate,
        })
    }

    fn parse_polygon(&mut self) -> Result<Polygon, quick_xml::Error> {
        let mut outer: LinearRing = LinearRing::default();
        let mut inner: Vec<LinearRing> = Vec::new();
        let mut altitude_mode = types::AltitudeMode::default();
        let mut extrude = false;
        let mut tesselate = false;

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
                    b"tesselate" => tesselate = self.parse_str()? == "1",
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
            tesselate,
        })
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
        let mut tesselate = false;

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"coordinates" => {
                        coords = self.parse_coords()?;
                    }
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.parse_str()?).unwrap()
                    }
                    b"extrude" => extrude = self.parse_str()? == "1",
                    b"tesselate" => tesselate = self.parse_str()? == "1",
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == end_tag {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(GeomProps {
            coords,
            altitude_mode,
            extrude,
            tesselate,
        })
    }

    fn parse_coords(&mut self) -> Result<Vec<Coord>, quick_xml::Error> {
        let e = self.reader.read_event(&mut self.buf)?;
        match e {
            Event::Text(e) => Ok(coords_from_str(
                &e.unescape_and_decode(&self.reader).expect("Error!"),
            )
            .expect("e")),
            _ => Err(quick_xml::Error::UnexpectedToken("f".to_string())),
        }
    }

    fn parse_str(&mut self) -> Result<String, quick_xml::Error> {
        let e = self.reader.read_event(&mut self.buf)?;
        match e {
            Event::Text(e) => Ok(e.unescape_and_decode(&self.reader).expect("Error")),
            _ => Err(quick_xml::Error::UnexpectedToken("f".to_string())),
        }
    }
}

// TODO: Might have to include everything here, all possible variables like tesselate
struct GeomProps {
    coords: Vec<Coord>,
    altitude_mode: types::AltitudeMode,
    extrude: bool,
    tesselate: bool,
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
                    tesselate: false
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
                    tesselate: false
                },
            _ => false,
        }))
    }
}
