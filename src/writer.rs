//! Module for writing KML types
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
    Alias, BalloonStyle, Coord, CoordType, Element, Folder, Geometry, Icon, IconStyle, Kml,
    LabelStyle, LineString, LineStyle, LinearRing, Link, LinkTypeIcon, ListStyle, Location,
    MultiGeometry, Orientation, Pair, Placemark, Point, PolyStyle, Polygon, ResourceMap, Scale,
    SchemaData, SimpleArrayData, SimpleData, Style, StyleMap,
};

/// Struct for managing writing KML
pub struct KmlWriter<W: Write, T: CoordType + FromStr + Default = f64> {
    writer: quick_xml::Writer<W>,
    _phantom: PhantomData<T>,
}

impl<'a, W, T> KmlWriter<W, T>
where
    W: Write,
    T: CoordType + FromStr + Default + fmt::Display,
{
    /// Creates `KmlWriter` from an input that implements `Write`
    ///
    /// # Example
    ///
    /// ```
    /// use kml::{Kml, KmlWriter, types::Point};
    ///
    /// let kml = Kml::Point(Point::new(1., 1., None));
    ///
    /// let mut buf = Vec::new();
    /// let mut writer = KmlWriter::<_, f64>::from_writer(&mut buf);
    /// ```
    pub fn from_writer(w: W) -> KmlWriter<W, T> {
        KmlWriter::new(quick_xml::Writer::new(w))
    }

    pub fn new(writer: quick_xml::Writer<W>) -> KmlWriter<W, T> {
        KmlWriter {
            writer,
            _phantom: PhantomData,
        }
    }

    /// Writes KML to a `Writer`
    ///
    /// # Example
    ///
    /// ```
    /// use kml::{Kml, KmlWriter, types::Point};
    ///
    /// let kml = Kml::Point(Point::new(1., 1., None));
    ///
    /// let mut buf = Vec::new();
    /// let mut writer = KmlWriter::from_writer(&mut buf);
    /// writer.write(&kml).unwrap();
    /// ```
    pub fn write(&mut self, kml: &Kml<T>) -> Result<(), Error> {
        self.write_kml(kml)
    }

    fn write_kml(&mut self, k: &Kml<T>) -> Result<(), Error> {
        match k {
            Kml::KmlDocument(d) => self.write_container("kml", &d.attrs, &d.elements)?,
            Kml::Scale(s) => self.write_scale(s)?,
            Kml::Orientation(o) => self.write_orientation(o)?,
            Kml::Point(p) => self.write_point(p)?,
            Kml::Location(l) => self.write_location(l)?,
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
            Kml::LinkTypeIcon(i) => self.write_link_type_icon(i)?,
            Kml::Link(l) => self.write_link(l)?,
            Kml::ResourceMap(r) => self.write_resource_map(r)?,
            Kml::Alias(a) => self.write_alias(a)?,
            Kml::SchemaData(s) => self.write_schema_data(s)?,
            Kml::SimpleArrayData(s) => self.write_simple_array_data(s)?,
            Kml::SimpleData(s) => self.write_simple_data(s)?,
            Kml::Document { attrs, elements } => {
                self.write_container("Document", attrs, elements)?
            }
            Kml::Folder(f) => self.write_folder(f)?,
            Kml::Element(e) => self.write_element(e)?,
        }
        Ok(())
    }

    fn write_scale(&mut self, scale: &Scale<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Scale").with_attributes(self.hash_map_as_attrs(&scale.attrs)),
        ))?;
        self.write_text_element("x", &scale.x.to_string())?;
        self.write_text_element("y", &scale.y.to_string())?;
        self.write_text_element("z", &scale.z.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Scale")))?)
    }

    fn write_orientation(&mut self, orientation: &Orientation<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Orientation")
                .with_attributes(self.hash_map_as_attrs(&orientation.attrs)),
        ))?;
        self.write_text_element("roll", &orientation.roll.to_string())?;
        self.write_text_element("tilt", &orientation.tilt.to_string())?;
        self.write_text_element("heading", &orientation.heading.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Orientation")))?)
    }

    fn write_point(&mut self, point: &Point<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Point").with_attributes(self.hash_map_as_attrs(&point.attrs)),
        ))?;
        self.write_text_element("extrude", if point.extrude { "1" } else { "0" })?;
        self.write_text_element("altitudeMode", &point.altitude_mode.to_string())?;
        self.write_text_element("coordinates", &point.coord.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Point")))?)
    }

    fn write_location(&mut self, location: &Location<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Location").with_attributes(self.hash_map_as_attrs(&location.attrs)),
        ))?;
        self.write_text_element("longitude", &location.longitude.to_string())?;
        self.write_text_element("latitude", &location.latitude.to_string())?;
        self.write_text_element("altitude", &location.altitude.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Location")))?)
    }

    fn write_line_string(&mut self, line_string: &LineString<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("LineString")
                .with_attributes(self.hash_map_as_attrs(&line_string.attrs)),
        ))?;
        // TODO: Avoid clone here?
        self.write_geom_props(GeomProps {
            coords: line_string.coords.clone(),
            altitude_mode: line_string.altitude_mode,
            extrude: line_string.extrude,
            tessellate: line_string.tessellate,
        })?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("LineString")))?)
    }

    fn write_linear_ring(&mut self, linear_ring: &LinearRing<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("LinearRing")
                .with_attributes(self.hash_map_as_attrs(&linear_ring.attrs)),
        ))?;
        self.write_geom_props(GeomProps {
            // TODO: Avoid clone if possible
            coords: linear_ring.coords.clone(),
            altitude_mode: linear_ring.altitude_mode,
            extrude: linear_ring.extrude,
            tessellate: linear_ring.tessellate,
        })?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("LinearRing")))?)
    }

    fn write_polygon(&mut self, polygon: &Polygon<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Polygon").with_attributes(self.hash_map_as_attrs(&polygon.attrs)),
        ))?;
        self.write_geom_props(GeomProps {
            coords: Vec::new(),
            altitude_mode: polygon.altitude_mode,
            extrude: polygon.extrude,
            tessellate: polygon.tessellate,
        })?;
        self.writer
            .write_event(Event::Start(BytesStart::new("outerBoundaryIs")))?;
        self.write_linear_ring(&polygon.outer)?;
        self.writer
            .write_event(Event::End(BytesEnd::new("outerBoundaryIs")))?;

        if !polygon.inner.is_empty() {
            self.writer
                .write_event(Event::Start(BytesStart::new("innerBoundaryIs")))?;
            for b in &polygon.inner {
                self.write_linear_ring(b)?;
            }
            self.writer
                .write_event(Event::End(BytesEnd::new("innerBoundaryIs")))?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Polygon")))?)
    }

    fn write_multi_geometry(&mut self, multi_geometry: &MultiGeometry<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("MultiGeometry")
                .with_attributes(self.hash_map_as_attrs(&multi_geometry.attrs)),
        ))?;

        for g in multi_geometry.geometries.iter() {
            self.write_geometry(g)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("MultiGeometry")))?)
    }

    fn write_placemark(&mut self, placemark: &Placemark<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Placemark").with_attributes(self.hash_map_as_attrs(&placemark.attrs)),
        ))?;
        if let Some(name) = &placemark.name {
            self.write_text_element("name", name)?;
        }
        if let Some(description) = &placemark.description {
            self.write_text_element("description", description)?;
        }
        for c in placemark.children.iter() {
            self.write_element(c)?;
        }
        if let Some(geometry) = &placemark.geometry {
            self.write_geometry(geometry)?;
        }
        if let Some(style_url) = &placemark.style_url {
            self.write_text_element("styleUrl", style_url)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Placemark")))?)
    }

    fn write_element(&mut self, e: &Element) -> Result<(), Error> {
        let start = BytesStart::new(&e.name).with_attributes(self.hash_map_as_attrs(&e.attrs));
        self.writer.write_event(Event::Start(start))?;
        if let Some(content) = &e.content {
            self.writer
                .write_event(Event::Text(BytesText::new(content)))?;
        }
        for c in e.children.iter() {
            self.write_element(c)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new(&e.name)))?)
    }

    fn write_folder(&mut self, folder: &Folder<T>) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Folder").with_attributes(self.hash_map_as_attrs(&folder.attrs)),
        ))?;
        if let Some(name) = &folder.name {
            self.write_text_element("name", name)?;
        }
        if let Some(description) = &folder.description {
            self.write_text_element("description", description)?;
        }
        for e in folder.elements.iter() {
            self.write_kml(e)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Folder")))?)
    }

    fn write_style(&mut self, style: &Style) -> Result<(), Error> {
        let attrs = if let Some(id) = &style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("Style").with_attributes(attrs),
        ))?;
        if let Some(balloon) = &style.balloon {
            self.write_balloon_style(balloon)?;
        }
        if let Some(icon) = &style.icon {
            self.write_icon_style(icon)?;
        }
        if let Some(label) = &style.label {
            self.write_label_style(label)?;
        }
        if let Some(line) = &style.line {
            self.write_line_style(line)?;
        }
        if let Some(poly) = &style.poly {
            self.write_poly_style(poly)?;
        }
        if let Some(list) = &style.list {
            self.write_list_style(list)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Style")))?)
    }

    fn write_style_map(&mut self, style_map: &StyleMap) -> Result<(), Error> {
        let attrs = if let Some(id) = &style_map.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&style_map.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("StyleMap").with_attributes(attrs),
        ))?;
        for p in style_map.pairs.iter() {
            self.write_pair(p)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("StyleMap")))?)
    }

    fn write_pair(&mut self, pair: &Pair) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Pair").with_attributes(self.hash_map_as_attrs(&pair.attrs)),
        ))?;
        self.write_text_element("key", &pair.key)?;
        self.write_text_element("styleUrl", &pair.style_url)?;
        Ok(self.writer.write_event(Event::End(BytesEnd::new("Pair")))?)
    }

    fn write_balloon_style(&mut self, balloon_style: &BalloonStyle) -> Result<(), Error> {
        let attrs = if let Some(id) = &balloon_style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&balloon_style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("BalloonStyle").with_attributes(attrs),
        ))?;
        if let Some(bg_color) = &balloon_style.bg_color {
            self.write_text_element("bgColor", bg_color)?;
        }
        self.write_text_element("textColor", &balloon_style.text_color)?;
        if let Some(text) = &balloon_style.text {
            self.write_text_element("text", text)?;
        }
        if !balloon_style.display {
            self.write_text_element("displayMode", "hide")?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("BalloonStyle")))?)
    }

    fn write_icon_style(&mut self, icon_style: &IconStyle) -> Result<(), Error> {
        let attrs = if let Some(id) = &icon_style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&icon_style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("IconStyle").with_attributes(attrs),
        ))?;
        self.write_text_element("scale", &icon_style.scale.to_string())?;
        self.write_text_element("heading", &icon_style.heading.to_string())?;
        if let Some(hot_spot) = &icon_style.hot_spot {
            self.writer
                .write_event(Event::Start(BytesStart::new("hotSpot").with_attributes(
                    vec![
                        ("x", &*hot_spot.x.to_string()),
                        ("y", &*hot_spot.y.to_string()),
                        ("xunits", &*hot_spot.xunits.to_string()),
                        ("yunits", &*hot_spot.yunits.to_string()),
                    ],
                )))?;
            self.writer
                .write_event(Event::End(BytesEnd::new("hotSpot")))?;
        }
        self.write_text_element("color", &icon_style.color)?;
        self.write_text_element("colorMode", &icon_style.color_mode.to_string())?;
        self.write_icon(&icon_style.icon)?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("IconStyle")))?)
    }

    fn write_icon(&mut self, icon: &Icon) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::new("Icon")))?;
        self.write_text_element("href", &icon.href)?;
        Ok(self.writer.write_event(Event::End(BytesEnd::new("Icon")))?)
    }

    fn write_label_style(&mut self, label_style: &LabelStyle) -> Result<(), Error> {
        let attrs = if let Some(id) = &label_style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&label_style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("LabelStyle").with_attributes(attrs),
        ))?;
        self.write_text_element("color", &label_style.color)?;
        self.write_text_element("colorMode", &label_style.color_mode.to_string())?;
        self.write_text_element("scale", &label_style.scale.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("LabelStyle")))?)
    }

    fn write_line_style(&mut self, line_style: &LineStyle) -> Result<(), Error> {
        let attrs = if let Some(id) = &line_style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&line_style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("LineStyle").with_attributes(attrs),
        ))?;
        self.write_text_element("color", &line_style.color)?;
        self.write_text_element("colorMode", &line_style.color_mode.to_string())?;
        self.write_text_element("width", &line_style.width.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("LineStyle")))?)
    }

    fn write_poly_style(&mut self, poly_style: &PolyStyle) -> Result<(), Error> {
        let attrs = if let Some(id) = &poly_style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&poly_style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("PolyStyle").with_attributes(attrs),
        ))?;
        self.write_text_element("color", &poly_style.color)?;
        self.write_text_element("colorMode", &poly_style.color_mode.to_string())?;
        self.write_text_element("fill", &poly_style.fill.to_string())?;
        self.write_text_element("outline", &poly_style.outline.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("PolyStyle")))?)
    }

    fn write_list_style(&mut self, list_style: &ListStyle) -> Result<(), Error> {
        let attrs = if let Some(id) = &list_style.id {
            vec![("id", id.as_ref())]
        } else {
            vec![]
        };
        let attrs: Vec<(&str, &str)> = attrs
            .into_iter()
            .chain(self.hash_map_as_attrs(&list_style.attrs))
            .collect();
        self.writer.write_event(Event::Start(
            BytesStart::new("ListStyle").with_attributes(attrs),
        ))?;
        self.write_text_element("bgColor", &list_style.bg_color)?;
        self.write_text_element("maxSnippetLines", &list_style.max_snippet_lines.to_string())?;
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("ListStyle")))?)
    }

    fn write_link_type_icon(&mut self, icon: &LinkTypeIcon) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Icon").with_attributes(self.hash_map_as_attrs(&icon.attrs)),
        ))?;
        if let Some(href) = &icon.href {
            self.write_text_element("href", href)?;
        }
        if let Some(refresh_mode) = &icon.refresh_mode {
            self.write_text_element("refreshMode", &refresh_mode.to_string())?;
        }
        self.write_text_element("refreshInterval", &icon.refresh_interval.to_string())?;
        if let Some(view_refresh_mode) = &icon.view_refresh_mode {
            self.write_text_element("viewRefreshMode", &view_refresh_mode.to_string())?;
        }
        self.write_text_element("viewRefreshTime", &icon.view_refresh_time.to_string())?;
        self.write_text_element("viewBoundScale", &icon.view_bound_scale.to_string())?;
        if let Some(view_format) = &icon.view_format {
            self.write_text_element("viewFormat", view_format)?;
        }
        if let Some(http_query) = &icon.http_query {
            self.write_text_element("httpQuery", http_query)?;
        }
        Ok(self.writer.write_event(Event::End(BytesEnd::new("Icon")))?)
    }

    fn write_link(&mut self, link: &Link) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Link").with_attributes(self.hash_map_as_attrs(&link.attrs)),
        ))?;
        if let Some(href) = &link.href {
            self.write_text_element("href", href)?;
        }
        if let Some(refresh_mode) = &link.refresh_mode {
            self.write_text_element("refreshMode", &refresh_mode.to_string())?;
        }
        self.write_text_element("refreshInterval", &link.refresh_interval.to_string())?;
        if let Some(view_refresh_mode) = &link.view_refresh_mode {
            self.write_text_element("viewRefreshMode", &view_refresh_mode.to_string())?;
        }
        self.write_text_element("viewRefreshTime", &link.view_refresh_time.to_string())?;
        self.write_text_element("viewBoundScale", &link.view_bound_scale.to_string())?;
        if let Some(view_format) = &link.view_format {
            self.write_text_element("viewFormat", view_format)?;
        }
        if let Some(http_query) = &link.http_query {
            self.write_text_element("httpQuery", http_query)?;
        }
        Ok(self.writer.write_event(Event::End(BytesEnd::new("Link")))?)
    }

    fn write_resource_map(&mut self, resource_map: &ResourceMap) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("ResourceMap")
                .with_attributes(self.hash_map_as_attrs(&resource_map.attrs)),
        ))?;
        for alias in resource_map.aliases.iter() {
            self.write_alias(alias)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("ResourceMap")))?)
    }

    fn write_alias(&mut self, alias: &Alias) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("Alias").with_attributes(self.hash_map_as_attrs(&alias.attrs)),
        ))?;
        if let Some(href) = &alias.target_href {
            self.write_text_element("targetHref", href)?;
        }
        if let Some(href) = &alias.source_href {
            self.write_text_element("sourceHref", href)?;
        }
        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("Alias")))?)
    }

    fn write_schema_data(&mut self, schema_data: &SchemaData) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new("SchemaData")
                .with_attributes(self.hash_map_as_attrs(&schema_data.attrs)),
        ))?;

        for value in schema_data.data.iter() {
            self.write_simple_data(value)?;
        }

        for value in schema_data.arrays.iter() {
            self.write_simple_array_data(value)?;
        }

        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("SchemaData")))?)
    }

    fn write_simple_array_data(
        &mut self,
        simple_array_data: &SimpleArrayData,
    ) -> Result<(), Error> {
        let filter_attrs = HashMap::from([("name".to_string(), simple_array_data.name.clone())]);
        self.writer.write_event(Event::Start(
            BytesStart::new("SimpleArrayData").with_attributes(
                self.hash_map_as_attrs_filtered(&simple_array_data.attrs, &filter_attrs),
            ),
        ))?;

        for value in simple_array_data.values.iter() {
            self.write_text_element("value", value)?;
        }

        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("SimpleArrayData")))?)
    }

    fn write_simple_data(&mut self, simple_data: &SimpleData) -> Result<(), Error> {
        let filter_attrs = HashMap::from([("name".to_string(), simple_data.name.clone())]);
        self.writer
            .write_event(Event::Start(BytesStart::new("SimpleData").with_attributes(
                self.hash_map_as_attrs_filtered(&simple_data.attrs, &filter_attrs),
            )))?;

        self.writer
            .write_event(Event::Text(BytesText::new(&simple_data.value)))?;

        Ok(self
            .writer
            .write_event(Event::End(BytesEnd::new("SimpleData")))?)
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
        self.write_text_element("extrude", if props.extrude { "1" } else { "0" })?;
        self.write_text_element("tessellate", if props.tessellate { "1" } else { "0" })?;
        self.write_text_element("altitudeMode", &props.altitude_mode.to_string())?;
        if !props.coords.is_empty() {
            self.write_text_element(
                "coordinates",
                &props
                    .coords
                    .iter()
                    .map(Coord::to_string)
                    .collect::<Vec<String>>()
                    .join("\n"),
            )?
        }
        Ok(())
    }

    fn write_container(
        &mut self,
        tag: &str,
        attrs: &HashMap<String, String>,
        elements: &[Kml<T>],
    ) -> Result<(), Error> {
        self.writer.write_event(Event::Start(
            BytesStart::new(tag).with_attributes(self.hash_map_as_attrs(attrs)),
        ))?;
        for e in elements.iter() {
            self.write_kml(e)?;
        }
        // Wrapping in Ok to coerce the quick_xml::Error type with ?
        Ok(self.writer.write_event(Event::End(BytesEnd::new(tag)))?)
    }

    fn write_text_element(&mut self, tag: &str, content: &str) -> Result<(), Error> {
        self.writer
            .write_event(Event::Start(BytesStart::new(tag)))?;
        self.writer
            .write_event(Event::Text(BytesText::new(content)))?;
        Ok(self.writer.write_event(Event::End(BytesEnd::new(tag)))?)
    }

    fn hash_map_as_attrs(&self, hash_map: &'a HashMap<String, String>) -> Vec<(&'a str, &'a str)> {
        hash_map
            .iter()
            .map(|(k, v)| (&k[..], &v[..]))
            .collect::<Vec<(&str, &str)>>()
    }

    fn hash_map_as_attrs_filtered(
        &self,
        hash_map: &'a HashMap<String, String>,
        filter_hash_map: &'a HashMap<String, String>,
    ) -> Vec<(&'a str, &'a str)> {
        // Filter out select props like id/name so that we include them first in order
        filter_hash_map
            .iter()
            .chain(
                hash_map
                    .iter()
                    .filter(|(k, _)| !filter_hash_map.contains_key(&k.to_string())),
            )
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
        KmlWriter::from_writer(&mut buf)
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
        assert_eq!("<Point><extrude>0</extrude><altitudeMode>relativeToGround</altitudeMode><coordinates>1,1,1</coordinates></Point>", kml.to_string());
    }

    #[test]
    fn test_write_location() {
        let kml = Kml::Location(Location {
            longitude: 17.27,
            latitude: -93.09,
            altitude: 350.1,
            ..Default::default()
        });
        let expected_string = "<Location>\
            <longitude>17.27</longitude>\
            <latitude>-93.09</latitude>\
            <altitude>350.1</altitude>\
        </Location>";
        assert_eq!(expected_string, kml.to_string());
    }

    #[test]
    fn test_write_link() {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "Some ID".to_string());

        let kml: Kml<f64> = Kml::Link(Link {
            href: Some("/path/to/local/resource".to_string()),
            refresh_mode: Some(types::RefreshMode::OnChange),
            view_refresh_mode: Some(types::ViewRefreshMode::OnStop),
            attrs,
            ..Default::default()
        });
        let expected_string = "<Link id=\"Some ID\">\
            <href>/path/to/local/resource</href>\
            <refreshMode>onChange</refreshMode>\
            <refreshInterval>4</refreshInterval>\
            <viewRefreshMode>onStop</viewRefreshMode>\
            <viewRefreshTime>4</viewRefreshTime>\
            <viewBoundScale>1</viewBoundScale>\
        </Link>";
        assert_eq!(expected_string, kml.to_string());
    }

    #[test]
    fn test_write_link_icon() {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "Some ID".to_string());

        let kml: Kml<f64> = Kml::LinkTypeIcon(LinkTypeIcon {
            href: Some("/path/to/local/resource".to_string()),
            refresh_mode: Some(types::RefreshMode::OnChange),
            view_refresh_mode: Some(types::ViewRefreshMode::OnStop),
            attrs,
            ..Default::default()
        });
        let expected_string = "<Icon id=\"Some ID\">\
            <href>/path/to/local/resource</href>\
            <refreshMode>onChange</refreshMode>\
            <refreshInterval>4</refreshInterval>\
            <viewRefreshMode>onStop</viewRefreshMode>\
            <viewRefreshTime>4</viewRefreshTime>\
            <viewBoundScale>1</viewBoundScale>\
        </Icon>";
        assert_eq!(expected_string, kml.to_string());
    }

    #[test]
    fn test_write_resource_map() {
        // Alias 1
        let mut alias1_attrs = HashMap::new();
        alias1_attrs.insert("id".to_string(), "Alias ID 1".to_string());

        let alias1 = Alias {
            target_href: Some("../images/foo1.jpg".to_string()),
            source_href: Some("in-geometry-file/foo1.jpg".to_string()),
            attrs: alias1_attrs,
        };

        // Alias 2
        let mut alias2_attrs = HashMap::new();
        alias2_attrs.insert("id".to_string(), "Alias ID 2".to_string());

        let alias2 = Alias {
            target_href: Some("../images/foo2.jpg".to_string()),
            source_href: Some("in-geometry-file/foo2.jpg".to_string()),
            attrs: alias2_attrs,
        };

        // ResourceMap
        let mut resource_map_attrs = HashMap::new();
        resource_map_attrs.insert("id".to_string(), "ResourceMap ID".to_string());

        let kml: Kml<f64> = Kml::ResourceMap(ResourceMap {
            aliases: vec![alias1, alias2],
            attrs: resource_map_attrs,
        });

        let expected_string = "<ResourceMap id=\"ResourceMap ID\">\
            <Alias id=\"Alias ID 1\">\
                <targetHref>../images/foo1.jpg</targetHref>\
                <sourceHref>in-geometry-file/foo1.jpg</sourceHref>\
            </Alias>\
            <Alias id=\"Alias ID 2\">\
                <targetHref>../images/foo2.jpg</targetHref>\
                <sourceHref>in-geometry-file/foo2.jpg</sourceHref>\
            </Alias>\
        </ResourceMap>";

        assert_eq!(expected_string, kml.to_string());

        // Test a ResourceMap with `None` for its `aliases` field writes zero Aliases
        assert_eq!(
            "<ResourceMap></ResourceMap>",
            Kml::ResourceMap::<f64>(ResourceMap {
                aliases: Vec::new(),
                attrs: HashMap::new(),
            })
            .to_string()
        );
    }

    #[test]
    fn test_write_alias() {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "Some ID".to_string());

        let kml: Kml<f64> = Kml::Alias(Alias {
            target_href: Some("../images/foo.jpg".to_string()),
            source_href: Some("in-geometry-file/foo.jpg".to_string()),
            attrs,
        });

        let expected_string = "<Alias id=\"Some ID\">\
            <targetHref>../images/foo.jpg</targetHref>\
            <sourceHref>in-geometry-file/foo.jpg</sourceHref>\
        </Alias>";
        assert_eq!(expected_string, kml.to_string());
    }

    #[test]
    fn test_write_schema_data() {
        let kml: Kml<f64> = Kml::SchemaData(SchemaData {
            data: vec![
                SimpleData {
                    name: "TrailHeadName".to_string(),
                    value: "Pi in the sky".to_string(),
                    attrs: [("anyAttribute".to_string(), "anySimpleType".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                },
                SimpleData {
                    name: "TrailLength".to_string(),
                    value: "3.14159".to_string(),
                    attrs: [("name".to_string(), "duplicate name attribute".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                },
            ],
            arrays: vec![
                SimpleArrayData {
                    name: "cadence".to_string(),
                    values: vec!["86".to_string(), "113".to_string(), "113".to_string()],
                    attrs: [("anyAttribute".to_string(), "anySimpleType".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                },
                SimpleArrayData {
                    name: "heartrate".to_string(),
                    values: vec!["181".to_string()],
                    ..Default::default()
                },
            ],
            attrs: [("schemaUrl".to_string(), "#TrailHeadTypeId".to_string())]
                .iter()
                .cloned()
                .collect(),
        });

        let expected_string = "<SchemaData schemaUrl=\"#TrailHeadTypeId\">\
            <SimpleData name=\"TrailHeadName\" anyAttribute=\"anySimpleType\">Pi in the sky</SimpleData>\
            <SimpleData name=\"TrailLength\">3.14159</SimpleData>\
            <SimpleArrayData name=\"cadence\" anyAttribute=\"anySimpleType\">\
                <value>86</value>\
                <value>113</value>\
                <value>113</value>\
            </SimpleArrayData>\
            <SimpleArrayData name=\"heartrate\">\
                <value>181</value>\
            </SimpleArrayData>\
        </SchemaData>";

        assert_eq!(expected_string, kml.to_string());
    }

    #[test]
    fn test_write_scale() {
        let kml = Kml::Scale(Scale {
            x: 3.5,
            y: 2.,
            ..Default::default()
        });
        let expected_string = "<Scale>\
            <x>3.5</x>\
            <y>2</y>\
            <z>1</z>\
        </Scale>";
        assert_eq!(expected_string, kml.to_string());
    }

    #[test]
    fn test_write_orientation() {
        let kml = Kml::Orientation(Orientation {
            roll: -170.279,
            tilt: 13.,
            heading: 45.07,
            ..Default::default()
        });
        let expected_string = "<Orientation>\
            <roll>-170.279</roll>\
            <tilt>13</tilt>\
            <heading>45.07</heading>\
        </Orientation>";
        assert_eq!(expected_string, kml.to_string());
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
            r#"<Polygon><extrude>0</extrude><tessellate>0</tessellate><altitudeMode>clampToGround</altitudeMode><outerBoundaryIs><LinearRing><extrude>0</extrude><tessellate>1</tessellate><altitudeMode>clampToGround</altitudeMode><coordinates>-1,2,0
-1.5,3,0
-1.5,2,0
-1,2,0</coordinates></LinearRing></outerBoundaryIs></Polygon>"#,
            kml.to_string()
        );
    }

    #[test]
    fn test_write_style_map() {
        let kml: Kml = Kml::StyleMap(StyleMap {
            id: Some("id".to_string()),
            attrs: HashMap::from([("test".to_string(), "test".to_string())]),
            ..Default::default()
        });

        assert_eq!(
            r#"<StyleMap id="id" test="test"></StyleMap>"#,
            kml.to_string()
        );
    }
}
