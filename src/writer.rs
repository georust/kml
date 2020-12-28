use std::fmt::{self, Debug};
use std::io::Write;
use std::marker::PhantomData;
use std::str;
use std::str::FromStr;

use num_traits::Float;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::errors::Error;
use crate::types::geom_props::GeomProps;
use crate::types::{
    Coord, Element, Geometry, Kml, LineString, LinearRing, MultiGeometry, Placemark, Point, Polygon,
};

pub struct KmlWriter<W: Write, T: Float + FromStr + Default + Debug = f64> {
    writer: quick_xml::Writer<W>,
    _phantom: PhantomData<T>,
}

impl<W, T> KmlWriter<W, T>
where
    W: Write,
    T: Float + FromStr + Default + Debug + fmt::Display,
{
    pub fn new(writer: quick_xml::Writer<W>) -> KmlWriter<W, T> {
        KmlWriter {
            writer,
            _phantom: PhantomData,
        }
    }

    pub fn write(&mut self, kml: &Kml<T>) -> Result<(), Error> {
        self.write_kml(kml)
    }

    fn write_kml(&mut self, k: &Kml<T>) -> Result<(), Error> {
        match k {
            Kml::KmlDocument(d) => self.write_container(b"kml", &d.elements)?,
            Kml::Point(p) => self.write_point(p)?,
            Kml::LineString(l) => self.write_line_string(l)?,
            Kml::LinearRing(l) => self.write_linear_ring(l)?,
            Kml::Polygon(p) => self.write_polygon(p)?,
            Kml::MultiGeometry(g) => self.write_multi_geometry(g)?,
            Kml::Placemark(p) => self.write_placemark(p)?,
            Kml::Document { elements } => self.write_container(b"Document", &elements)?,
            Kml::Folder { elements } => self.write_container(b"Folder", &elements)?,
            Kml::Element(e) => self.write_element(e)?,
        }

        Ok(())
    }

    fn write_point(&mut self, point: &Point<T>) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(b"Point".to_vec())))?;
        self.write_text_element(b"coordinates", &point.coord.to_string())?;
        self.write_text_element(b"altitudeMode", &point.altitude_mode.to_string())?;
        self.write_text_element(b"extrude", if point.extrude { "1" } else { "0" })?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::owned(b"Point".to_vec())))?)
    }

    fn write_line_string(&mut self, line_string: &LineString<T>) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(b"LineString".to_vec())))?;
        // TODO: Avoid clone here?
        self.write_geom_props(GeomProps {
            coords: line_string.coords.clone(),
            altitude_mode: line_string.altitude_mode,
            extrude: line_string.extrude,
            tessellate: line_string.tessellate,
        })?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::owned(b"LineString".to_vec())))?)
    }

    fn write_linear_ring(&mut self, linear_ring: &LinearRing<T>) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(b"LinearRing".to_vec())))?;
        self.write_geom_props(GeomProps {
            // TODO: Remove clone, convert GeomProps to tuple params?
            coords: linear_ring.coords.clone(),
            altitude_mode: linear_ring.altitude_mode,
            extrude: linear_ring.extrude,
            tessellate: linear_ring.tessellate,
        })?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::owned(b"LinearRing".to_vec())))?)
    }

    fn write_polygon(&mut self, polygon: &Polygon<T>) -> Result<(), Error> {
        // TODO: Clean up start creation
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(b"Polygon".to_vec())))?;
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(
                b"outerBoundaryIs".to_vec(),
            )))?;
        self.write_linear_ring(&polygon.outer)?;
        self.writer
            .write_event(Event::End(BytesEnd::borrowed(b"outerBoundaryIs")))?;

        if !polygon.inner.is_empty() {
            self.writer
                .write_event(Event::Start(BytesStart::owned_name(
                    b"innerBoundaryIs".to_vec(),
                )))?;
            for b in &polygon.inner {
                self.write_linear_ring(b)?;
            }
            self.writer
                .write_event(Event::End(BytesEnd::borrowed(b"innerBoundaryIs")))?;
        }
        self.write_geom_props(GeomProps {
            coords: Vec::new(),
            altitude_mode: polygon.altitude_mode,
            extrude: polygon.extrude,
            tessellate: polygon.tessellate,
        })?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"Polygon")))?)
    }

    fn write_multi_geometry(&mut self, multi_geometry: &MultiGeometry<T>) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(
                b"MultiGeometry".to_vec(),
            )))?;

        for g in multi_geometry.0.iter() {
            self.write_geometry(g)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::owned(b"MultiGeometry".to_vec())))?)
    }

    fn write_placemark(&mut self, placemark: &Placemark<T>) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(b"Placemark".to_vec())))?;
        if let Some(name) = &placemark.name {
            self.write_text_element(b"name", &name)?;
        }
        if let Some(description) = &placemark.description {
            self.write_text_element(b"description", &description)?;
        }
        if let Some(geometry) = &placemark.geometry {
            self.write_geometry(geometry)?;
        }
        for c in placemark.children.iter() {
            self.write_element(c)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"Placemark")))?)
    }

    fn write_element(&mut self, e: &Element) -> Result<(), Error> {
        let start = BytesStart::borrowed_name(e.name.as_bytes()).with_attributes(
            e.attrs
                .iter()
                .map(|(k, v)| (&k[..], &v[..]))
                .collect::<Vec<(&str, &str)>>(),
        );
        self.writer.write_event(Event::Start(start))?;
        if let Some(content) = &e.content {
            self.writer
                .write_event(Event::Text(BytesText::from_plain_str(&content)))?;
        }
        for c in e.children.iter() {
            self.write_element(c)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(e.name.as_bytes())))?)
    }

    fn write_geometry(&mut self, geometry: &Geometry<T>) -> Result<(), Error> {
        match geometry {
            Geometry::Point(p) => self.write_point(p),
            Geometry::LineString(l) => self.write_line_string(l),
            Geometry::LinearRing(l) => self.write_linear_ring(l),
            Geometry::Polygon(p) => self.write_polygon(p),
            Geometry::MultiGeometry(g) => self.write_multi_geometry(g),
            _ => Ok(()),
        }
    }

    fn write_geom_props(&mut self, props: GeomProps<T>) -> Result<(), Error> {
        if !props.coords.is_empty() {
            self.write_text_element(
                b"coordinates",
                &props
                    .coords
                    .iter()
                    .map(Coord::to_string)
                    .collect::<Vec<String>>()
                    .join("\n"),
            )?;
        }
        self.write_text_element(b"altitudeMode", &props.altitude_mode.to_string())?;
        self.write_text_element(b"extrude", if props.extrude { "1" } else { "0" })?;
        self.write_text_element(b"tessellate", if props.tessellate { "1" } else { "0" })
    }

    fn write_container(&mut self, tag: &[u8], elements: &[Kml<T>]) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(tag)))?;
        for e in elements.iter() {
            self.write_kml(e)?;
        }
        // Wrapping in Ok to coerce the quick_xml::Error type with ?
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(tag)))?)
    }

    fn write_text_element(&mut self, tag: &[u8], content: &str) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(tag)))?;
        self.writer
            .write_event(Event::Text(BytesText::from_plain_str(content)))?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(tag)))?)
    }
}

impl<T> fmt::Display for Kml<T>
where
    T: Float + Default + Debug + FromStr + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = Vec::new();
        KmlWriter::new(quick_xml::Writer::new(&mut buf))
            .write(self)
            .map_err(|_| fmt::Error)
            .and_then(|_| f.write_str(str::from_utf8(&buf).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types;

    #[test]
    fn test_write_point() {
        let kml = Kml::Point(Point {
            coord: Coord {
                x: 1.,
                y: 1.,
                z: Some(1.),
            },
            altitude_mode: types::AltitudeMode::RelativeToGround,
            extrude: false,
        });
        assert_eq!("<Point><coordinates>1,1,1</coordinates><altitudeMode>relativeToGround</altitudeMode><extrude>0</extrude></Point>", kml.to_string());
    }

    #[test]
    fn test_write_polygon() {
        let kml = Kml::Polygon(Polygon {
            outer: LinearRing {
                coords: vec![
                    Coord {
                        x: -1.,
                        y: 2.,
                        z: Some(0.),
                    },
                    Coord {
                        x: -1.5,
                        y: 3.,
                        z: Some(0.),
                    },
                    Coord {
                        x: -1.5,
                        y: 2.,
                        z: Some(0.),
                    },
                    Coord {
                        x: -1.,
                        y: 2.,
                        z: Some(0.),
                    },
                ],
                extrude: false,
                tessellate: true,
                altitude_mode: types::AltitudeMode::ClampToGround,
            },
            inner: vec![],
            extrude: false,
            tessellate: false,
            altitude_mode: types::AltitudeMode::ClampToGround,
        });

        assert_eq!(
            r#"<Polygon><outerBoundaryIs><LinearRing><coordinates>-1,2,0
-1.5,3,0
-1.5,2,0
-1,2,0</coordinates><altitudeMode>clampToGround</altitudeMode><extrude>0</extrude><tessellate>1</tessellate></LinearRing></outerBoundaryIs><altitudeMode>clampToGround</altitudeMode><extrude>0</extrude><tessellate>0</tessellate></Polygon>"#,
            kml.to_string()
        );
    }
}
