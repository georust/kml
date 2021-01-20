use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

/// `kml:Style`, [12.2](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#798) in the KML
/// specification
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Style {
    pub id: String,
    pub balloon: Option<BalloonStyle>,
    pub icon: Option<IconStyle>,
    pub label: Option<LabelStyle>,
    pub line: Option<LineStyle>,
    pub poly: Option<PolyStyle>,
    pub list: Option<ListStyle>,
}

/// `kml:StyleMap`, [12.3](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#811) in the KML
/// specification
#[derive(Clone, Default, Debug, PartialEq)]
pub struct StyleMap {
    pub id: String,
    pub pairs: Vec<Pair>,
}

/// `kml:Pair`, [12.4](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#819) in the KML
/// specification
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Pair {
    pub key: String,
    pub style_url: String,
    pub attrs: HashMap<String, String>,
}

/// `kml:BalloonStyle`, [12.7](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#841) in the
/// KML specification
#[derive(Clone, Debug, PartialEq)]
pub struct BalloonStyle {
    pub id: String,
    pub bg_color: Option<String>,
    pub text_color: String,
    pub text: Option<String>,
    pub display: bool,
}

impl Default for BalloonStyle {
    fn default() -> BalloonStyle {
        BalloonStyle {
            id: "".to_string(),
            bg_color: None,
            text_color: "ffffffff".to_string(),
            text: None,
            display: true,
        }
    }
}

/// `kml:colorMode`, [12.11](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#879) in the
/// KML specification
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorMode {
    Default,
    Random,
}

impl Default for ColorMode {
    fn default() -> ColorMode {
        ColorMode::Default
    }
}

impl FromStr for ColorMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "random" => Ok(Self::Random),
            v => Err(Error::InvalidColorMode(v.to_string())),
        }
    }
}

impl fmt::Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Default => "default",
                Self::Random => "random",
            }
        )
    }
}

/// `kml:IconStyle`, [12.12](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#883) in the
/// KML specification
#[derive(Clone, Debug, PartialEq)]
pub struct IconStyle {
    pub id: String,
    pub scale: f64,
    pub heading: f64,
    pub hot_spot: Option<(f64, f64)>,
    pub icon: Icon,
    pub color: String,
    pub color_mode: ColorMode,
}

impl Default for IconStyle {
    fn default() -> IconStyle {
        IconStyle {
            id: "".to_string(),
            scale: 1.0,
            heading: 0.0,
            hot_spot: None,
            icon: Icon::default(),
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
        }
    }
}

/// `kml:Icon`, [12.13](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#900) in the KML
/// specification.
///
/// Implements on `kml:BasicLinkType`
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Icon {
    pub href: String,
}

/// `kml:LabelStyle`, [12.14](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#909) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct LabelStyle {
    pub id: String,
    pub color: String,
    pub color_mode: ColorMode,
    pub scale: f64,
}

impl Default for LabelStyle {
    fn default() -> LabelStyle {
        LabelStyle {
            id: "".to_string(),
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            scale: 1.0,
        }
    }
}

/// `kml:LineStyle`, [12.15](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#917) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct LineStyle {
    pub id: String,
    pub color: String,
    pub color_mode: ColorMode,
    pub width: f64,
}

impl Default for LineStyle {
    fn default() -> LineStyle {
        LineStyle {
            id: "".to_string(),
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            width: 1.0,
        }
    }
}

/// `kml:PolyStyle`, [12.16](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#927) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct PolyStyle {
    pub id: String,
    pub color: String,
    pub color_mode: ColorMode,
    pub fill: bool,
    pub outline: bool,
}

impl Default for PolyStyle {
    fn default() -> PolyStyle {
        PolyStyle {
            id: "".to_string(),
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            fill: true,
            outline: true,
        }
    }
}

/// `kml:listItemType`, [12.18](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#955) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub enum ListItemType {
    Check,
    CheckOffOnly,
    CheckHideChildren,
    RadioFolder,
}

impl Default for ListItemType {
    fn default() -> ListItemType {
        ListItemType::Check
    }
}

impl FromStr for ListItemType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "check" => Ok(Self::Check),
            "checkOffOnly" => Ok(Self::CheckOffOnly),
            "checkHideChildren" => Ok(Self::CheckHideChildren),
            "radioFolder" => Ok(Self::RadioFolder),
            v => Err(Error::InvalidListItemType(v.to_string())),
        }
    }
}

impl fmt::Display for ListItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Check => "check",
                Self::CheckOffOnly => "checkOffOnly",
                Self::CheckHideChildren => "checkHideChildren",
                Self::RadioFolder => "radioFolder",
            }
        )
    }
}

/// `kml:ListStyle`, [12.17](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#940) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct ListStyle {
    pub id: String,
    pub bg_color: String,
    pub max_snippet_lines: u32,
    pub list_item_type: ListItemType,
}

impl Default for ListStyle {
    fn default() -> ListStyle {
        ListStyle {
            id: "".to_string(),
            bg_color: "ffffffff".to_string(),
            max_snippet_lines: 2,
            list_item_type: ListItemType::default(),
        }
    }
}
