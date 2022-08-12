//! Module for reading KML sources into Rust types
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::Path;
use std::str;
use std::str::FromStr;

use num_traits::{Float, One, Zero};
use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};

use crate::errors::Error;
use crate::types::geom_props::GeomProps;
use crate::types::{
    self, coords_from_str, Alias, BalloonStyle, ColorMode, Coord, CoordType, Element, Geometry,
    Icon, IconStyle, Kml, KmlDocument, KmlVersion, LabelStyle, LineString, LineStyle, LinearRing,
    Link, LinkTypeIcon, ListStyle, Location, MultiGeometry, Orientation, Pair, Placemark, Point,
    PolyStyle, Polygon, RefreshMode, ResourceMap, Scale, Style, StyleMap, Units, Vec2,
    ViewRefreshMode,
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
    /// Parse KML from string
    ///
    /// # Example
    ///
    /// ```
    /// use kml::{Kml, KmlReader};
    ///
    /// let point_str = "<Point><coordinates>1,1,1</coordinates></Point>";
    /// let kml_point: Kml<f64> = KmlReader::from_string(point_str).read().unwrap();
    /// ```
    pub fn from_string(s: &str) -> KmlReader<&[u8], T> {
        KmlReader::<&[u8], T>::from_xml_reader(quick_xml::Reader::<&[u8]>::from_str(s))
    }
}

impl<T> KmlReader<BufReader<File>, T>
where
    T: CoordType + FromStr + Default,
{
    /// Read KML from a file path
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::Path;
    /// use kml::KmlReader;
    ///
    /// let poly_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    ///     .join("tests")
    ///     .join("fixtures")
    ///     .join("polygon.kml");
    /// let mut kml_reader = KmlReader::<_, f64>::from_path(poly_path).unwrap();
    /// let kml = kml_reader.read().unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<KmlReader<BufReader<File>, T>, Error> {
        Ok(KmlReader::<BufReader<File>, T>::from_xml_reader(
            quick_xml::Reader::from_file(path)?,
        ))
    }
}

impl<B: BufRead, T> KmlReader<B, T>
where
    T: CoordType + FromStr + Default,
{
    /// Read from any generic reader type
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

    /// Read content into [`Kml`](enum.Kml.html)
    ///
    /// # Example
    ///
    /// ```
    /// use kml::{Kml, KmlReader};
    ///
    /// let point_str = "<Point><coordinates>1,1,1</coordinates></Point>";
    /// let kml_point: Kml<f64> = KmlReader::from_string(point_str).read().unwrap();
    /// ```
    pub fn read(&mut self) -> Result<Kml<T>, Error> {
        let mut result = self.read_elements()?;
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

    fn read_elements(&mut self) -> Result<Vec<Kml<T>>, Error> {
        let mut elements: Vec<Kml<T>> = Vec::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    let attrs = Self::read_attrs(e.attributes());
                    match e.local_name() {
                        b"kml" => elements.push(Kml::KmlDocument(self.read_kml_document()?)),
                        b"Scale" => elements.push(Kml::Scale(self.read_scale(attrs)?)),
                        b"Orientation" => {
                            elements.push(Kml::Orientation(self.read_orientation(attrs)?))
                        }
                        b"Point" => elements.push(Kml::Point(self.read_point(attrs)?)),
                        b"Location" => elements.push(Kml::Location(self.read_location(attrs)?)),
                        b"LineString" => {
                            elements.push(Kml::LineString(self.read_line_string(attrs)?))
                        }
                        b"LinearRing" => {
                            elements.push(Kml::LinearRing(self.read_linear_ring(attrs)?))
                        }
                        b"Polygon" => elements.push(Kml::Polygon(self.read_polygon(attrs)?)),
                        b"MultiGeometry" => {
                            elements.push(Kml::MultiGeometry(self.read_multi_geometry(attrs)?))
                        }
                        b"Placemark" => elements.push(Kml::Placemark(self.read_placemark(attrs)?)),
                        b"Document" => elements.push(Kml::Document {
                            attrs,
                            elements: self.read_elements()?,
                        }),
                        b"Folder" => elements.push(Kml::Folder {
                            attrs,
                            elements: self.read_elements()?,
                        }),
                        b"Style" => elements.push(Kml::Style(self.read_style(attrs)?)),
                        b"StyleMap" => elements.push(Kml::StyleMap(self.read_style_map(attrs)?)),
                        b"Pair" => elements.push(Kml::Pair(self.read_pair(attrs)?)),
                        b"BalloonStyle" => {
                            elements.push(Kml::BalloonStyle(self.read_balloon_style(attrs)?))
                        }
                        b"IconStyle" => elements.push(Kml::IconStyle(self.read_icon_style(attrs)?)),
                        b"Link" => elements.push(Kml::Link(self.read_link(attrs)?)),
                        b"Icon" => {
                            elements.push(Kml::LinkTypeIcon(self.read_link_type_icon(attrs)?))
                        }
                        b"ResourceMap" => {
                            elements.push(Kml::ResourceMap(self.read_resource_map(attrs)?))
                        }
                        b"Alias" => elements.push(Kml::Alias(self.read_alias(attrs)?)),
                        b"LabelStyle" => {
                            elements.push(Kml::LabelStyle(self.read_label_style(attrs)?))
                        }
                        b"LineStyle" => elements.push(Kml::LineStyle(self.read_line_style(attrs)?)),
                        b"PolyStyle" => elements.push(Kml::PolyStyle(self.read_poly_style(attrs)?)),
                        b"ListStyle" => elements.push(Kml::ListStyle(self.read_list_style(attrs)?)),
                        _ => {
                            let start = e.to_owned();
                            elements.push(Kml::Element(self.read_element(&start, attrs)?));
                        }
                    };
                }
                Event::End(ref mut e) => match e.local_name() {
                    b"Folder" | b"Document" => break,
                    _ => {}
                },
                Event::Decl(_) | Event::CData(_) | Event::Empty(_) | Event::Text(_) => {}
                Event::Eof => break,
                _ => return Err(Error::InvalidInput),
            };
        }

        Ok(elements)
    }

    fn read_kml_document(&mut self) -> Result<KmlDocument<T>, Error> {
        // TODO: Should parse version, change version based on NS
        Ok(KmlDocument {
            elements: self.read_elements()?,
            ..Default::default()
        })
    }

    fn read_scale(&mut self, attrs: HashMap<String, String>) -> Result<Scale<T>, Error> {
        let mut x = One::one();
        let mut y = One::one();
        let mut z = One::one();

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"x" => x = self.read_float()?,
                    b"y" => y = self.read_float()?,
                    b"z" => z = self.read_float()?,
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Scale" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(Scale { x, y, z, attrs })
    }

    fn read_orientation(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<Orientation<T>, Error> {
        let mut roll = Zero::zero();
        let mut tilt = Zero::zero();
        let mut heading = Zero::zero();

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"roll" => roll = self.read_float()?,
                    b"tilt" => tilt = self.read_float()?,
                    b"heading" => heading = self.read_float()?,
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Orientation" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(Orientation {
            roll,
            tilt,
            heading,
            attrs,
        })
    }

    fn read_point(&mut self, attrs: HashMap<String, String>) -> Result<Point<T>, Error> {
        let mut props = self.read_geom_props(b"Point")?;
        Ok(Point {
            coord: props.coords.remove(0),
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            attrs,
        })
    }

    fn read_location(&mut self, attrs: HashMap<String, String>) -> Result<Location<T>, Error> {
        let mut longitude = Zero::zero();
        let mut latitude = Zero::zero();
        let mut altitude = Zero::zero();

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"longitude" => longitude = self.read_float()?,
                    b"latitude" => latitude = self.read_float()?,
                    b"altitude" => altitude = self.read_float()?,
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Location" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(Location {
            longitude,
            latitude,
            altitude,
            attrs,
        })
    }

    fn read_line_string(&mut self, attrs: HashMap<String, String>) -> Result<LineString<T>, Error> {
        let props = self.read_geom_props(b"LineString")?;
        Ok(LineString {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
            attrs,
        })
    }

    fn read_linear_ring(&mut self, attrs: HashMap<String, String>) -> Result<LinearRing<T>, Error> {
        let props = self.read_geom_props(b"LinearRing")?;
        Ok(LinearRing {
            coords: props.coords,
            altitude_mode: props.altitude_mode,
            extrude: props.extrude,
            tessellate: props.tessellate,
            attrs,
        })
    }

    fn read_polygon(&mut self, attrs: HashMap<String, String>) -> Result<Polygon<T>, Error> {
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
                        let mut outer_ring = self.read_boundary(b"outerBoundaryIs")?;
                        if outer_ring.is_empty() {
                            return Err(Error::InvalidGeometry(
                                "Polygon must have an outer boundary".to_string(),
                            ));
                        }
                        outer = outer_ring.remove(0);
                    }
                    b"innerBoundaryIs" => inner = self.read_boundary(b"innerBoundaryIs")?,
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.read_str()?)?
                    }
                    b"extrude" => extrude = self.read_str()? == "1",
                    b"tessellate" => tessellate = self.read_str()? == "1",
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

    fn read_multi_geometry(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<MultiGeometry<T>, Error> {
        let mut geometries: Vec<Geometry<T>> = Vec::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    let attrs = Self::read_attrs(e.attributes());
                    match e.local_name() {
                        b"Point" => geometries.push(Geometry::Point(self.read_point(attrs)?)),
                        b"LineString" => {
                            geometries.push(Geometry::LineString(self.read_line_string(attrs)?))
                        }
                        b"LinearRing" => {
                            geometries.push(Geometry::LinearRing(self.read_linear_ring(attrs)?))
                        }
                        b"Polygon" => geometries.push(Geometry::Polygon(self.read_polygon(attrs)?)),
                        b"MultiGeometry" => geometries
                            .push(Geometry::MultiGeometry(self.read_multi_geometry(attrs)?)),
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

    fn read_placemark(&mut self, attrs: HashMap<String, String>) -> Result<Placemark<T>, Error> {
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut geometry: Option<Geometry<T>> = None;
        let mut children: Vec<Element> = Vec::new();

        loop {
            let e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref e) => {
                    let attrs = Self::read_attrs(e.attributes());
                    match e.local_name() {
                        b"name" => name = Some(self.read_str()?),
                        b"description" => description = Some(self.read_str()?),
                        b"Point" => geometry = Some(Geometry::Point(self.read_point(attrs)?)),
                        b"LineString" => {
                            geometry = Some(Geometry::LineString(self.read_line_string(attrs)?))
                        }
                        b"LinearRing" => {
                            geometry = Some(Geometry::LinearRing(self.read_linear_ring(attrs)?))
                        }
                        b"Polygon" => geometry = Some(Geometry::Polygon(self.read_polygon(attrs)?)),
                        b"MultiGeometry" => {
                            geometry =
                                Some(Geometry::MultiGeometry(self.read_multi_geometry(attrs)?))
                        }
                        _ => {
                            let start = e.to_owned();
                            let start_attrs = Self::read_attrs(start.attributes());
                            children.push(self.read_element(&start, start_attrs)?);
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

    fn read_style(&mut self, attrs: HashMap<String, String>) -> Result<Style, Error> {
        let mut style = Style::default();
        if let Some(id_str) = attrs.get("id") {
            style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    let attrs = Self::read_attrs(e.attributes());
                    match e.local_name() {
                        b"BalloonStyle" => style.balloon = Some(self.read_balloon_style(attrs)?),
                        b"IconStyle" => style.icon = Some(self.read_icon_style(attrs)?),
                        b"LabelStyle" => style.label = Some(self.read_label_style(attrs)?),
                        b"LineStyle" => style.line = Some(self.read_line_style(attrs)?),
                        b"PolyStyle" => style.poly = Some(self.read_poly_style(attrs)?),
                        b"ListStyle" => style.list = Some(self.read_list_style(attrs)?),
                        _ => {}
                    }
                }
                Event::End(ref mut e) => {
                    if e.local_name() == b"Style" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(style)
    }

    fn read_style_map(&mut self, attrs: HashMap<String, String>) -> Result<StyleMap, Error> {
        let mut style_map = StyleMap::default();
        if let Some(id_str) = attrs.get("id") {
            style_map.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    if e.local_name() == b"Pair" {
                        let pair_attrs = Self::read_attrs(e.attributes());
                        style_map.pairs.push(self.read_pair(pair_attrs)?);
                    }
                }
                Event::End(ref mut e) => {
                    if e.local_name() == b"StyleMap" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(style_map)
    }

    fn read_pair(&mut self, attrs: HashMap<String, String>) -> Result<Pair, Error> {
        let mut pair = Pair {
            attrs,
            ..Pair::default()
        };

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"key" => pair.key = self.read_str()?,
                    b"styleUrl" => pair.style_url = self.read_str()?,
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Pair" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(pair)
    }

    fn read_icon_style(&mut self, attrs: HashMap<String, String>) -> Result<IconStyle, Error> {
        let mut icon_style = IconStyle::default();
        if let Some(id_str) = attrs.get("id") {
            icon_style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"scale" => icon_style.scale = self.read_float()?,
                    b"heading" => icon_style.heading = self.read_float()?,
                    b"hot_spot" => {
                        let hot_spot_attrs = Self::read_attrs(e.attributes());
                        let x_val = hot_spot_attrs.get("x");
                        let y_val = hot_spot_attrs.get("y");
                        let xunits = hot_spot_attrs.get("xunits");
                        let yunits = hot_spot_attrs.get("yunits");
                        if let (Some(x_str), Some(y_str)) = (x_val, y_val) {
                            let x: f64 = x_str
                                .parse()
                                .map_err(|_| Error::NumParse(x_str.to_string()))?;
                            let y: f64 = y_str
                                .parse()
                                .map_err(|_| Error::NumParse(y_str.to_string()))?;
                            let xunits = xunits
                                .map_or_else(|| Ok(Units::default()), |units| units.parse())?;
                            let yunits = yunits
                                .map_or_else(|| Ok(Units::default()), |units| units.parse())?;
                            icon_style.hot_spot = Some(Vec2 {
                                x,
                                y,
                                xunits,
                                yunits,
                            });
                        }
                    }
                    b"Icon" => icon_style.icon = self.read_basic_link_type_icon()?,
                    b"color" => icon_style.color = self.read_str()?,
                    b"colorMode" => {
                        icon_style.color_mode = self.read_str()?.parse::<ColorMode>()?
                    }
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"IconStyle" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(icon_style)
    }

    fn read_basic_link_type_icon(&mut self) -> Result<Icon, Error> {
        let mut href = String::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    if e.local_name() == b"href" {
                        href = self.read_str()?;
                    }
                }
                Event::End(ref mut e) => {
                    if e.local_name() == b"Icon" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(Icon { href })
    }

    fn read_link_type_icon(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<LinkTypeIcon, Error> {
        let mut icon = LinkTypeIcon {
            attrs,
            ..Default::default()
        };
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"href" => icon.href = Some(self.read_str()?),
                    b"refreshMode" => {
                        icon.refresh_mode = Some(RefreshMode::from_str(&self.read_str()?)?);
                    }
                    b"refreshInterval" => icon.refresh_interval = self.read_float()?,
                    b"viewRefreshMode" => {
                        icon.view_refresh_mode = Some(ViewRefreshMode::from_str(&self.read_str()?)?)
                    }
                    b"viewRefreshTime" => icon.view_refresh_time = self.read_float()?,
                    b"viewBoundScale" => icon.view_bound_scale = self.read_float()?,
                    b"viewFormat" => icon.view_format = Some(self.read_str()?),
                    b"httpQuery" => icon.http_query = Some(self.read_str()?),
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Icon" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(icon)
    }

    fn read_link(&mut self, attrs: HashMap<String, String>) -> Result<Link, Error> {
        let mut link = Link {
            attrs,
            ..Default::default()
        };
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"href" => link.href = Some(self.read_str()?),
                    b"refreshMode" => {
                        link.refresh_mode = Some(RefreshMode::from_str(&self.read_str()?)?);
                    }
                    b"refreshInterval" => link.refresh_interval = self.read_float()?,
                    b"viewRefreshMode" => {
                        link.view_refresh_mode = Some(ViewRefreshMode::from_str(&self.read_str()?)?)
                    }
                    b"viewRefreshTime" => link.view_refresh_time = self.read_float()?,
                    b"viewBoundScale" => link.view_bound_scale = self.read_float()?,
                    b"viewFormat" => link.view_format = Some(self.read_str()?),
                    b"httpQuery" => link.http_query = Some(self.read_str()?),
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"Link" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(link)
    }

    fn read_resource_map(&mut self, attrs: HashMap<String, String>) -> Result<ResourceMap, Error> {
        let mut resource_map = ResourceMap {
            attrs,
            ..Default::default()
        };

        let mut aliases = Vec::new();

        loop {
            let e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(e) => {
                    if e.local_name() == b"Alias" {
                        let attrs = Self::read_attrs(e.attributes());
                        if let Ok(alias) = self.read_alias(attrs) {
                            aliases.push(alias);
                        }
                    }
                }
                Event::End(e) => {
                    if e.local_name() == b"ResourceMap" {
                        break;
                    }
                }
                _ => break,
            }
        }

        resource_map.aliases = aliases;

        Ok(resource_map)
    }

    fn read_alias(&mut self, attrs: HashMap<String, String>) -> Result<Alias, Error> {
        let mut alias = Alias {
            attrs,
            ..Default::default()
        };

        loop {
            let e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(e) => match e.local_name() {
                    b"targetHref" => alias.target_href = Some(self.read_str()?),
                    b"sourceHref" => alias.source_href = Some(self.read_str()?),
                    _ => {}
                },
                Event::End(e) => {
                    if e.local_name() == b"Alias" {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(alias)
    }

    fn read_balloon_style(
        &mut self,
        attrs: HashMap<String, String>,
    ) -> Result<BalloonStyle, Error> {
        let mut balloon_style = BalloonStyle::default();
        if let Some(id_str) = attrs.get("id") {
            balloon_style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"bgColor" => balloon_style.bg_color = Some(self.read_str()?),
                    b"textColor" => balloon_style.text_color = self.read_str()?,
                    b"text" => balloon_style.text = Some(self.read_str()?),
                    b"displayMode" => balloon_style.display = self.read_str()? != "hide",
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"BalloonStyle" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(balloon_style)
    }

    fn read_label_style(&mut self, attrs: HashMap<String, String>) -> Result<LabelStyle, Error> {
        let mut label_style = LabelStyle::default();
        if let Some(id_str) = attrs.get("id") {
            label_style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"color" => label_style.color = self.read_str()?,
                    b"colorMode" => {
                        label_style.color_mode = self.read_str()?.parse::<ColorMode>()?;
                    }
                    b"scale" => label_style.scale = self.read_float()?,
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"LabelStyle" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(label_style)
    }

    fn read_line_style(&mut self, attrs: HashMap<String, String>) -> Result<LineStyle, Error> {
        let mut line_style = LineStyle::default();
        if let Some(id_str) = attrs.get("id") {
            line_style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"color" => line_style.color = self.read_str()?,
                    b"colorMode" => {
                        line_style.color_mode = self.read_str()?.parse::<ColorMode>()?;
                    }
                    b"width" => line_style.width = self.read_float()?,
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"LineStyle" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(line_style)
    }

    fn read_list_style(&mut self, attrs: HashMap<String, String>) -> Result<ListStyle, Error> {
        let mut list_style = ListStyle::default();
        if let Some(id_str) = attrs.get("id") {
            list_style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"bgColor" => list_style.bg_color = self.read_str()?,
                    b"maxSnippetLines" => {
                        let line_str = self.read_str()?;
                        list_style.max_snippet_lines = line_str
                            .parse::<u32>()
                            .map_err(|_| Error::NumParse(line_str))?;
                    }
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"ListStyle" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(list_style)
    }

    fn read_poly_style(&mut self, attrs: HashMap<String, String>) -> Result<PolyStyle, Error> {
        let mut poly_style = PolyStyle::default();
        if let Some(id_str) = attrs.get("id") {
            poly_style.id = id_str.to_string();
        }
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"color" => poly_style.color = self.read_str()?,
                    b"colorMode" => {
                        poly_style.color_mode = self.read_str()?.parse::<ColorMode>()?;
                    }
                    b"fill" => {
                        let fill_str = self.read_str()?;
                        poly_style.fill = fill_str != "false" && fill_str != "0"
                    }
                    b"outline" => {
                        let outline_str = self.read_str()?;
                        poly_style.outline = outline_str != "false" && outline_str != "0"
                    }
                    _ => {}
                },
                Event::End(ref mut e) => {
                    if e.local_name() == b"PolyStyle" {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(poly_style)
    }

    fn read_element(
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
                    let start_attrs = Self::read_attrs(start.attributes());
                    element
                        .children
                        .push(self.read_element(&start, start_attrs)?);
                }
                Event::Text(ref mut e) => {
                    element.content = Some(
                        e.unescape_and_decode(&self.reader)
                            .unwrap_or_else(|_| String::from_utf8_lossy(e.escaped()).to_string()),
                    )
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

    fn read_boundary(&mut self, end_tag: &[u8]) -> Result<Vec<LinearRing<T>>, Error> {
        let mut boundary: Vec<LinearRing<T>> = Vec::new();
        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => {
                    let attrs = Self::read_attrs(e.attributes());
                    if e.local_name() == b"LinearRing" {
                        boundary.push(self.read_linear_ring(attrs)?);
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

    fn read_geom_props(&mut self, end_tag: &[u8]) -> Result<GeomProps<T>, Error> {
        let mut coords: Vec<Coord<T>> = Vec::new();
        let mut altitude_mode = types::AltitudeMode::default();
        let mut extrude = false;
        let mut tessellate = false;

        loop {
            let mut e = self.reader.read_event(&mut self.buf)?;
            match e {
                Event::Start(ref mut e) => match e.local_name() {
                    b"coordinates" => {
                        coords = coords_from_str(&self.read_str()?)?;
                    }
                    b"altitudeMode" => {
                        altitude_mode = types::AltitudeMode::from_str(&self.read_str()?)?
                    }
                    b"extrude" => extrude = self.read_str()? == "1",
                    b"tessellate" => tessellate = self.read_str()? == "1",
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

    fn read_float<F: Float + FromStr>(&mut self) -> Result<F, Error> {
        let float_str = self.read_str()?;
        float_str
            .parse::<F>()
            .map_err(|_| Error::NumParse(float_str))
    }

    fn read_str(&mut self) -> Result<String, Error> {
        let e = self.reader.read_event(&mut self.buf)?;
        match e {
            Event::Text(e) | Event::CData(e) => Ok(e
                .unescape_and_decode(&self.reader)
                .unwrap_or_else(|_| String::from_utf8_lossy(e.escaped()).to_string())),
            Event::End(_) => Ok("".to_string()),
            e => Err(Error::InvalidXmlEvent(format!("{:?}", e))),
        }
    }

    fn read_attrs(attrs: Attributes) -> HashMap<String, String> {
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
        KmlReader::<&[u8], T>::from_string(s).read()
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
    fn test_parse_location() {
        let poly_str = r#"<Location>
            <longitude>39.55</longitude>
            <latitude>-118.98</latitude>
            <altitude>1223</altitude>
        </Location>"#;
        let mut r = KmlReader::from_string(poly_str);

        let p: Kml = r.read().unwrap();
        assert_eq!(
            p,
            Kml::Location(Location {
                longitude: 39.55,
                latitude: -118.98,
                altitude: 1223.,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_read_link() {
        let kml_str = r#"<Link id="Some ID">
            <href>/path/to/local/resource</href>
            <refreshMode>onChange</refreshMode>
            <refreshInterval>4</refreshInterval>
            <viewRefreshMode>onStop</viewRefreshMode>
            <viewRefreshTime>4</viewRefreshTime>
            <viewBoundScale>1</viewBoundScale>
            <viewFormat></viewFormat>
        </Link>"#;

        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "Some ID".to_string());

        let l: Kml = kml_str.parse().unwrap();
        assert_eq!(
            l,
            Kml::Link(Link {
                href: Some("/path/to/local/resource".to_string()),
                refresh_mode: Some(types::RefreshMode::OnChange),
                view_refresh_mode: Some(types::ViewRefreshMode::OnStop),
                view_format: Some(String::new()),
                attrs,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_read_link_type_icon() {
        let kml_str = r#"<Icon id="Some ID">
            <href>/path/to/local/resource</href>
            <refreshMode>onChange</refreshMode>
            <refreshInterval>4</refreshInterval>
            <viewRefreshMode>onStop</viewRefreshMode>
            <viewRefreshTime>4</viewRefreshTime>
            <viewBoundScale>1</viewBoundScale>
            <viewFormat></viewFormat>
        </Icon>"#;

        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "Some ID".to_string());

        let l: Kml = kml_str.parse().unwrap();
        assert_eq!(
            l,
            Kml::LinkTypeIcon(LinkTypeIcon {
                href: Some("/path/to/local/resource".to_string()),
                refresh_mode: Some(types::RefreshMode::OnChange),
                view_refresh_mode: Some(types::ViewRefreshMode::OnStop),
                view_format: Some(String::new()),
                attrs,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_read_resource_map() {
        let kml_str = r#"<ResourceMap id="ResourceMap ID">
            <Alias id="Alias ID 1">
                <targetHref>../images/foo1.jpg</targetHref>
                <sourceHref>in-geometry-file/foo1.jpg</sourceHref>
            </Alias>
            <Alias id="Alias ID 2">
                <targetHref>../images/foo2.jpg</targetHref>
                <sourceHref>in-geometry-file/foo2.jpg</sourceHref>
            </Alias>
        </ResourceMap>"#;

        // Expected Alias 1
        let mut alias1_attrs = HashMap::new();
        alias1_attrs.insert("id".to_string(), "Alias ID 1".to_string());

        let alias1 = Alias {
            target_href: Some("../images/foo1.jpg".to_string()),
            source_href: Some("in-geometry-file/foo1.jpg".to_string()),
            attrs: alias1_attrs,
        };

        // Expected Alias 2
        let mut alias2_attrs = HashMap::new();
        alias2_attrs.insert("id".to_string(), "Alias ID 2".to_string());

        let alias2 = Alias {
            target_href: Some("../images/foo2.jpg".to_string()),
            source_href: Some("in-geometry-file/foo2.jpg".to_string()),
            attrs: alias2_attrs,
        };

        // Expected ResourceMap
        let mut resource_map_attrs = HashMap::new();
        resource_map_attrs.insert("id".to_string(), "ResourceMap ID".to_string());

        assert_eq!(
            kml_str.parse::<Kml>().unwrap(),
            Kml::ResourceMap(ResourceMap {
                aliases: vec![alias1, alias2],
                attrs: resource_map_attrs,
            })
        );

        // Test a ResourceMap with no Aliases has `None` for its `aliases` field
        assert_eq!(
            "<ResourceMap></ResourceMap>".parse::<Kml>().unwrap(),
            Kml::ResourceMap(ResourceMap {
                aliases: Vec::new(),
                attrs: HashMap::new(),
            })
        );
    }

    #[test]
    fn test_read_alias() {
        let kml_str = r#"<Alias id="Some ID">
            <targetHref>../images/foo.jpg</targetHref>
            <sourceHref>in-geometry-file/foo.jpg</sourceHref>
        </Alias>"#;

        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), "Some ID".to_string());

        let a: Kml = kml_str.parse().unwrap();
        assert_eq!(
            a,
            Kml::Alias(Alias {
                target_href: Some("../images/foo.jpg".to_string()),
                source_href: Some("in-geometry-file/foo.jpg".to_string()),
                attrs,
            })
        );
    }

    #[test]
    fn test_parse_scale() {
        let kml_str = r#"<Scale>
            <x>1.2</x>
            <y>3.5</y>
            <z>2.5</z>
        </Scale>"#;
        let s: Kml = kml_str.parse().unwrap();
        assert_eq!(
            s,
            Kml::Scale(Scale {
                x: 1.2,
                y: 3.5,
                z: 2.5,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_parse_orientation() {
        let kml_str = r#"<Orientation>
            <heading>45.01</heading>
            <tilt>-10.02</tilt>
            <roll>0.0</roll>
        </Orientation>"#;
        let l: Kml = kml_str.parse().unwrap();
        assert_eq!(
            l,
            Kml::Orientation(Orientation {
                roll: 0.,
                tilt: -10.02,
                heading: 45.01,
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

        let p: Kml = r.read().unwrap();
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
    fn test_read_str_lossy() {
        let kml_str = r#"
            <Placemark>
            <name><![CDATA[Test & Test]]></name>
            <description>1¼ miles</description>
            <Point>
            <coordinates>
                -1.0,1.0,0
            </coordinates>
            </Point>
        </Placemark>"#;
        let p: Kml = kml_str.parse().unwrap();
        assert!(matches!(p, Kml::Placemark(_)));
        let placemark: Placemark = match p {
            Kml::Placemark(p) => Some(p),
            _ => None,
        }
        .unwrap();
        assert_eq!(placemark.name, Some("Test & Test".to_string()));
        assert_eq!(placemark.description, Some("1¼ miles".to_string()));
    }

    #[test]
    fn test_parse_sibling_folders() {
        let kml_str = r#"
    <Folder>
        <name>Folder 1</name>
    </Folder>
    <Folder>
        <name>Folder 2</name>
    </Folder>
    "#;
        let f: Kml = kml_str.parse().unwrap();
        assert!(matches!(f, Kml::KmlDocument(_)));

        let doc: Option<KmlDocument> = match f {
            Kml::KmlDocument(d) => Some(d),
            _ => None,
        };
        let doc = doc.unwrap();

        assert_eq!(doc.elements.len(), 2);
        assert!(doc.elements.iter().all(|e| matches!(
            e,
            Kml::Folder {
                attrs: _,
                elements: _
            }
        )));
    }

    #[test]
    fn test_parse_doc_with_sibling_folders() {
        let kml_str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <kml xmlns="http://www.opengis.net/kml/2.2">
    <Document>
    <Folder>
        <name>Folder 1</name>
    </Folder>
    <Folder>
        <name>Folder 2</name>
    </Folder>
    </Document>
    </kml>
    "#;
        let f: Kml = kml_str.parse().unwrap();
        assert!(matches!(f, Kml::KmlDocument(_)));

        let elements: Option<Vec<Kml<_>>> = match f {
            Kml::KmlDocument(d) => match &d.elements[0] {
                Kml::Document { attrs: _, elements } => Some(elements.to_vec()),
                _ => None,
            },
            _ => None,
        };

        let elements = elements.unwrap();
        assert_eq!(elements.len(), 2);
        assert!(elements.iter().all(|e| matches!(
            e,
            Kml::Folder {
                attrs: _,
                elements: _
            }
        )));
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
