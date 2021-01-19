use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::marker::PhantomData;
use std::str;
use std::str::FromStr;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use crate::errors::Error;
use crate::types::geom_props::GeomProps;
use crate::types::{
    BalloonStyle, Coord, CoordType, Element, Geometry, Icon, IconStyle, Kml, LabelStyle,
    LineString, LineStyle, LinearRing, ListStyle, MultiGeometry, Pair, Placemark, Point, PolyStyle,
    Polygon, Style, StyleMap,
};

pub struct KmlWriter<W: Write, T: CoordType + FromStr + Default = f64> {
    writer: quick_xml::Writer<W>,
    _phantom: PhantomData<T>,
}

impl<'a, W, T> KmlWriter<W, T>
where
    W: Write,
    T: CoordType + FromStr + Default + fmt::Display,
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
            Kml::KmlDocument(d) => self.write_container(b"kml", &d.attrs, &d.elements)?,
            Kml::Point(p) => self.write_point(p)?,
            Kml::LineString(l) => self.write_line_string(l)?,
            Kml::LinearRing(l) => self.write_linear_ring(l)?,
            Kml::Polygon(p) => self.write_polygon(p)?,
            Kml::MultiGeometry(g) => self.write_multi_geometry(g)?,
            Kml::Placemark(p) => self.write_placemark(p)?,
            Kml::Style(s) => self.write_style(s)?,
            Kml::StyleMap(s) => self.write_style_map(s)?,
            Kml::Pair(p) => self.write_pair(p)?,
            Kml::BalloonStyle(b) => self.write_balloon_style(b)?,
            Kml::IconStyle(i) => self.write_icon_style(i)?,
            Kml::Icon(i) => self.write_icon(i)?,
            Kml::LabelStyle(l) => self.write_label_style(l)?,
            Kml::LineStyle(l) => self.write_line_style(l)?,
            Kml::PolyStyle(p) => self.write_poly_style(p)?,
            Kml::ListStyle(l) => self.write_list_style(l)?,
            Kml::Document { attrs, elements } => {
                self.write_container(b"Document", attrs, &elements)?
            }
            Kml::Folder { attrs, elements } => self.write_container(b"Folder", attrs, &elements)?,
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
            // TODO: Avoid clone if possible
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
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"Polygon".to_vec())
                .with_attributes(self.hash_map_as_attrs(&polygon.attrs)),
        ))?;
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
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"MultiGeometry".to_vec())
                .with_attributes(self.hash_map_as_attrs(&multi_geometry.attrs)),
        ))?;

        for g in multi_geometry.geometries.iter() {
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
        let start = BytesStart::borrowed_name(e.name.as_bytes())
            .with_attributes(self.hash_map_as_attrs(&e.attrs));
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

    fn write_style(&mut self, style: &Style) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"Style".to_vec()).with_attributes(vec![("id", &*style.id)]),
        ))?;
        if let Some(balloon) = &style.balloon {
            self.write_balloon_style(&balloon)?;
        }
        if let Some(icon) = &style.icon {
            self.write_icon_style(&icon)?;
        }
        if let Some(label) = &style.label {
            self.write_label_style(&label)?;
        }
        if let Some(line) = &style.line {
            self.write_line_style(&line)?;
        }
        if let Some(poly) = &style.poly {
            self.write_poly_style(&poly)?;
        }
        if let Some(list) = &style.list {
            self.write_list_style(&list)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"Style")))?)
    }

    fn write_style_map(&mut self, style_map: &StyleMap) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"StyleMap".to_vec())
                .with_attributes(vec![("id", &*style_map.id)]),
        ))?;
        for p in style_map.pairs.iter() {
            self.write_pair(p)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"StyleMap")))?)
    }

    fn write_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"Pair".to_vec())
                .with_attributes(self.hash_map_as_attrs(&pair.attrs)),
        ))?;
        self.write_text_element(b"key", &pair.key)?;
        self.write_text_element(b"styleUrl", &pair.style_url)?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"Pair")))?)
    }

    fn write_balloon_style(&mut self, balloon_style: &BalloonStyle) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"BalloonStyle".to_vec())
                .with_attributes(vec![("id", &*balloon_style.id)]),
        ))?;
        if let Some(bg_color) = &balloon_style.bg_color {
            self.write_text_element(b"bgColor", &bg_color)?;
        }
        self.write_text_element(b"textColor", &balloon_style.text_color)?;
        if let Some(text) = &balloon_style.text {
            self.write_text_element(b"text", &text)?;
        }
        if !balloon_style.display {
            self.write_text_element(b"displayMode", "hide")?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"BalloonStyle")))?)
    }

    fn write_icon_style(&mut self, icon_style: &IconStyle) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"IconStyle".to_vec())
                .with_attributes(vec![("id", &*icon_style.id)]),
        ))?;
        self.write_text_element(b"scale", &icon_style.scale.to_string())?;
        self.write_text_element(b"heading", &icon_style.heading.to_string())?;
        if let Some(hot_spot) = icon_style.hot_spot {
            self.writer.write_event(Event::Start(
                BytesStart::owned_name(b"hotSpot".to_vec()).with_attributes(vec![
                    ("x", &*hot_spot.0.to_string()),
                    ("y", &*hot_spot.1.to_string()),
                ]),
            ))?;
            self.writer
                .write_event(Event::End(BytesEnd::borrowed(b"hotSpot")))?;
        }
        self.write_text_element(b"color", &icon_style.color)?;
        self.write_text_element(b"colorMode", &icon_style.color_mode.to_string())?;
        self.write_icon(&icon_style.icon)?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"IconStyle")))?)
    }

    fn write_icon(&mut self, icon: &Icon) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::owned_name(b"Icon".to_vec())))?;
        self.write_text_element(b"href", &icon.href)?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"Icon")))?)
    }

    fn write_label_style(&mut self, label_style: &LabelStyle) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"LabelStyle".to_vec())
                .with_attributes(vec![("id", &*label_style.id)]),
        ))?;
        self.write_text_element(b"color", &label_style.color)?;
        self.write_text_element(b"colorMode", &label_style.color_mode.to_string())?;
        self.write_text_element(b"scale", &label_style.scale.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"LabelStyle")))?)
    }

    fn write_line_style(&mut self, line_style: &LineStyle) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"LineStyle".to_vec())
                .with_attributes(vec![("id", &*line_style.id)]),
        ))?;
        self.write_text_element(b"color", &line_style.color)?;
        self.write_text_element(b"colorMode", &line_style.color_mode.to_string())?;
        self.write_text_element(b"width", &line_style.width.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"LineStyle")))?)
    }

    fn write_poly_style(&mut self, poly_style: &PolyStyle) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"PolyStyle".to_vec())
                .with_attributes(vec![("id", &*poly_style.id)]),
        ))?;
        self.write_text_element(b"color", &poly_style.color)?;
        self.write_text_element(b"colorMode", &poly_style.color_mode.to_string())?;
        self.write_text_element(b"fill", &poly_style.fill.to_string())?;
        self.write_text_element(b"outline", &poly_style.outline.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"PolyStyle")))?)
    }

    fn write_list_style(&mut self, list_style: &ListStyle) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(b"ListStyle".to_vec())
                .with_attributes(vec![("id", &*list_style.id)]),
        ))?;
        self.write_text_element(b"bgColor", &list_style.bg_color)?;
        self.write_text_element(
            b"maxSnippetLines",
            &list_style.max_snippet_lines.to_string(),
        )?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::borrowed(b"ListStyle")))?)
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

    fn write_container(
        &mut self,
        tag: &[u8],
        attrs: &HashMap<String, String>,
        elements: &[Kml<T>],
    ) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::owned_name(tag).with_attributes(self.hash_map_as_attrs(attrs)),
        ))?;
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

    fn hash_map_as_attrs(&self, hash_map: &'a HashMap<String, String>) -> Vec<(&'a str, &'a str)> {
        hash_map
            .iter()
            .map(|(k, v)| (&k[..], &v[..]))
            .collect::<Vec<(&str, &str)>>()
    }
}

impl<T> fmt::Display for Kml<T>
where
    T: CoordType + Default + FromStr + fmt::Display,
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
            ..Default::default()
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
                tessellate: true,
                ..Default::default()
            },
            inner: vec![],
            ..Default::default()
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
