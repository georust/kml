use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::Path;
use std::str;
use std::str::FromStr;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};

use crate::errors::Error;
use crate::types::geom_props::GeomProps;
use crate::types::{
    self, coords_from_str, Coord, CoordType, Element, Geometry, Kml, KmlDocument, KmlVersion,
    LineString, LinearRing, MultiGeometry, Placemark, Point, Polygon,
};

/// Main struct for reading KML documents
pub struct KmlReader<B: BufRead, T: CoordType + FromStr + Default = f64> {
    reader: quick_xml::Reader<B>,
    buf: Vec<u8>,
    _version: KmlVersion, // TODO: How to incorporate this so it can be set before parsing?
    _phantom: PhantomData<T>,
}

impl<'a, T> KmlReader<&'a [u8], T>
where
    T: CoordType + FromStr + Default,
{
    pub fn from_string(s: &str) -> KmlReader<&[u8], T> {
        KmlReader::<&[u8], T>::from_xml_reader(quick_xml::Reader::<&[u8]>::from_str(s))
    }
}

impl<T> KmlReader<BufReader<File>, T>
where
    T: CoordType + FromStr + Default,
{
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<KmlReader<BufReader<File>, T>, Error> {
        Ok(KmlReader::<BufReader<File>, T>::from_xml_reader(
            quick_xml::Reader::from_file(path)?,
        ))
    }
}

impl<B: BufRead, T> KmlReader<B, T>
where
    T: CoordType + FromStr + Default,
{
    pub fn from_reader(r: B) -> KmlReader<B, T> {
        KmlReader::<B, T>::from_xml_reader(quick_xml::Reader::from_reader(r))
    }

    fn from_xml_reader(mut reader: quick_xml::Reader<B>) -> KmlReader<B, T> {
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        KmlReader {
            reader,
            buf: Vec::new(),
            _version: KmlVersion::Unknown,
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
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    let attrs = Self::parse_attrs(e.attributes());
                    match e.local_name() {
                        b"kml" => elements.push(Kml::KmlDocument(self.parse_kml_document()?)),
                        b"Point" => elements.push(Kml::Point(self.parse_point(attrs)?)),
                        b"LineString" => {
                            elements.push(Kml::LineString(self.parse_line_string(attrs)?))
                        }
                        b"LinearRing" => {
                            elements.push(Kml::LinearRing(self.parse_linear_ring(attrs)?))
                        }
                        b"Polygon" => elements.push(Kml::Polygon(self.parse_polygon(attrs)?)),
                        b"MultiGeometry" => {
                            elements.push(Kml::MultiGeometry(self.parse_multi_geometry(attrs)?))
                        }
                        b"Placemark" => elements.push(Kml::Placemark(self.parse_placemark(attrs)?)),
                        b"Document" => elements.push(Kml::Document {
                            attrs,
                            elements: self.parse_elements()?,
                        }),
                        b"Folder" => elements.push(Kml::Folder {
                            attrs,
                            elements: self.parse_elements()?,
                        }),
                        _ => {
                            let start = e.to_owned();
                            elements.push(Kml::Element(self.parse_element(&start, attrs)?));
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

    fn parse_point(&mut self, attrs: HashMap<String, String>) -> Result<Point<T>, Error> {
        let mut props = self.parse_geom_props(b"Point")?;
        Ok(Point {
            coord: props.coords.remove(0),
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            attrs,
        })
    }

    fn parse_line_string(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<LineString<T>, Error> {
        let props = self.parse_geom_props(b"LineString")?;
        Ok(LineString {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
            attrs,
        })
    }

    fn parse_linear_ring(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<LinearRing<T>, Error> {
        let props = self.parse_geom_props(b"LinearRing")?;
        Ok(LinearRing {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
            attrs,
        })
    }

    fn parse_polygon(&mut self, attrs: HashMap<String, String>) -> Result<Polygon<T>, Error> {
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
                        let mut outer_ring = self.parse_boundary(b"outerBoundaryIs")?;
                        if outer_ring.is_empty() {
                            return Err(Error::InvalidGeometry(
                                "Polygon must have an outer boundary".to_string(),
                            ));
                        }
                        outer = outer_ring.remove(0);
                    }
                    b"innerBoundaryIs" => inner = self.parse_boundary(b"innerBoundaryIs")?,
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.parse_str()?)?
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
            attrs,
        })
    }

    fn parse_multi_geometry(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<MultiGeometry<T>, Error> {
        let mut geometries: Vec<Geometry<T>> = Vec::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    let attrs = Self::parse_attrs(e.attributes());
                    match e.local_name() {
                        b"Point" => geometries.push(Geometry::Point(self.parse_point(attrs)?)),
                        b"LineString" => {
                            geometries.push(Geometry::LineString(self.parse_line_string(attrs)?))
                        }
                        b"LinearRing" => {
                            geometries.push(Geometry::LinearRing(self.parse_linear_ring(attrs)?))
                        }
                        b"Polygon" => {
                            geometries.push(Geometry::Polygon(self.parse_polygon(attrs)?))
                        }
                        b"MultiGeometry" => geometries
                            .push(Geometry::MultiGeometry(self.parse_multi_geometry(attrs)?)),
                        _ => {}
                    }
                }
                Event::End(ref mut e) => {
                    if e.local_name() == b"MultiGeometry" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(MultiGeometry { geometries, attrs })
    }

    fn parse_placemark(&mut self, attrs: HashMap<String, String>) -> Result<Placemark<T>, Error> {
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut geometry: Option<Geometry<T>> = None;
        let mut children: Vec<Element> = Vec::new();

        loop {
            let e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    let attrs = Self::parse_attrs(e.attributes());
                    match e.local_name() {
                        b"name" => name = Some(self.parse_str()?),
                        b"description" => description = Some(self.parse_str()?),
                        b"Point" => geometry = Some(Geometry::Point(self.parse_point(attrs)?)),
                        b"LineString" => {
                            geometry = Some(Geometry::LineString(self.parse_line_string(attrs)?))
                        }
                        b"LinearRing" => {
                            geometry = Some(Geometry::LinearRing(self.parse_linear_ring(attrs)?))
                        }
                        b"Polygon" => {
                            geometry = Some(Geometry::Polygon(self.parse_polygon(attrs)?))
                        }
                        b"MultiGeometry" => {
                            geometry =
                                Some(Geometry::MultiGeometry(self.parse_multi_geometry(attrs)?))
                        }
                        _ => {
                            let start = e.to_owned();
                            let start_attrs = Self::parse_attrs(start.attributes());
                            children.push(self.parse_element(&start, start_attrs)?);
                        }
                    }
                }
                Event::End(ref e) => {
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
            attrs,
            children,
        })
    }

    fn parse_element(
        &mut self,
        start: &BytesStart,
        attrs: HashMap<String, String>,
    ) -> Result<Element, Error> {
        let mut element = Element::default();
        let tag = start.local_name();
        element.name = str::from_utf8(tag).unwrap().to_string();
        element.attrs = attrs;
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(e) => {
                    let start = e.to_owned();
                    let start_attrs = Self::parse_attrs(start.attributes());
                    element
                        .children
                        .push(self.parse_element(&start, start_attrs)?);
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
                    let attrs = Self::parse_attrs(e.attributes());
                    if e.local_name() == b"LinearRing" {
                        boundary.push(self.parse_linear_ring(attrs)?);
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
                        coords = coords_from_str(&self.parse_str()?)?;
                    }
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.parse_str()?)?
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
        if coords.is_empty() {
            Err(Error::InvalidGeometry(
                "Geometry must contain coordinates element".to_string(),
            ))
        } else {
            Ok(GeomProps {
                coords,
                altitude_mode,
                extrude,
                tessellate,
            })
        }
    }

    fn parse_str(&mut self) -> Result<String, Error> {
        let e = self.reader.read_event(&mut self.buf)?;
        match e {
            Event::Text(e) | Event::CData(e) => {
                Ok(e.unescape_and_decode(&self.reader).expect("Error"))
            }
            e => Err(Error::InvalidXmlEvent(format!("{:?}", e))),
        }
    }

    fn parse_attrs(attrs: Attributes) -> HashMap<String, String> {
        attrs
            .filter_map(Result::ok)
            .map(|a| {
                (
                    str::from_utf8(a.key).unwrap().to_string(),
                    str::from_utf8(&a.value).unwrap().to_string(),
                )
            })
            .collect()
    }
}

impl<T> FromStr for Kml<T>
where
    T: CoordType + FromStr + Default,
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
        assert_eq!(
            p,
            Kml::Point(Point {
                coord: Coord {
                    x: 1.,
                    y: 1.,
                    z: Some(1.)
                },
                altitude_mode: types::AltitudeMode::RelativeToGround,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_parse_line_string() {
        let kml_str = r#"<LineString>
            <coordinates>1,1 2,1 3,1</coordinates>
            <altitudeMode>relativeToGround</altitudeMode>
        </LineString>"#;
        let l: Kml = kml_str.parse().unwrap();
        assert_eq!(
            l,
            Kml::LineString(LineString {
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
                ..Default::default()
            })
        );
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
        assert_eq!(
            p,
            Kml::Polygon(Polygon {
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
                    tessellate: true,
                    ..Default::default()
                },
                inner: vec![],
                ..Default::default()
            })
        );
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
                    ..Default::default()
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
                    ..Default::default()
                },
            _ => false,
        }))
    }

    #[test]
    fn test_parse() {
        let kml_str = include_str!("../tests/fixtures/sample.kml");

        assert!(matches!(
            Kml::<f64>::from_str(kml_str).unwrap(),
            Kml::KmlDocument(_)
        ))
    }
}
