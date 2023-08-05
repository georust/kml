use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

use crate::types::Vec2;

/// `kml:Style`, [12.2](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#798) in the KML
/// specification
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Style {
    pub id: Option<String>,
    pub balloon: Option<BalloonStyle>,
    pub icon: Option<IconStyle>,
    pub label: Option<LabelStyle>,
    pub line: Option<LineStyle>,
    pub poly: Option<PolyStyle>,
    pub list: Option<ListStyle>,
    pub attrs: HashMap<String, String>,
}

/// `kml:StyleMap`, [12.3](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#811) in the KML
/// specification
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct StyleMap {
    pub id: Option<String>,
    pub pairs: Vec<Pair>,
    pub attrs: HashMap<String, String>,
}

/// `kml:Pair`, [12.4](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#819) in the KML
/// specification
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Pair {
    pub key: String,
    pub style_url: String,
    pub attrs: HashMap<String, String>,
}

/// `kml:BalloonStyle`, [12.7](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#841) in the
/// KML specification
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BalloonStyle {
    pub id: Option<String>,
    pub bg_color: Option<String>,
    pub text_color: String,
    pub text: Option<String>,
    pub display: bool,
    pub attrs: HashMap<String, String>,
}

impl Default for BalloonStyle {
    fn default() -> BalloonStyle {
        BalloonStyle {
            id: None,
            bg_color: None,
            text_color: "ffffffff".to_string(),
            text: None,
            display: true,
            attrs: HashMap::new(),
        }
    }
}

/// `kml:colorMode`, [12.11](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#879) in the
/// KML specification
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ColorMode {
    #[default]
    Normal,
    Random,
}

impl FromStr for ColorMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
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
                Self::Normal => "normal",
                Self::Random => "random",
            }
        )
    }
}

/// `kml:IconStyle`, [12.12](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#883) in the
/// KML specification
#[derive(Clone, Debug, PartialEq)]
pub struct IconStyle {
    pub id: Option<String>,
    pub scale: f64,
    pub heading: f64,
    pub hot_spot: Option<Vec2>,
    pub icon: Icon,
    pub color: String,
    pub color_mode: ColorMode,
    pub attrs: HashMap<String, String>,
}

impl Default for IconStyle {
    fn default() -> IconStyle {
        IconStyle {
            id: None,
            scale: 1.0,
            heading: 0.0,
            hot_spot: None,
            icon: Icon::default(),
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            attrs: HashMap::new(),
        }
    }
}

/// `kml:Icon`, [12.13](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#900) in the KML
/// specification.
///
/// Implements on `kml:BasicLinkType`
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Icon {
    pub href: String,
    pub attrs: HashMap<String, String>,
}

/// `kml:LabelStyle`, [12.14](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#909) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct LabelStyle {
    pub id: Option<String>,
    pub color: String,
    pub color_mode: ColorMode,
    pub scale: f64,
    pub attrs: HashMap<String, String>,
}

impl Default for LabelStyle {
    fn default() -> LabelStyle {
        LabelStyle {
            id: None,
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            scale: 1.0,
            attrs: HashMap::new(),
        }
    }
}

/// `kml:LineStyle`, [12.15](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#917) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct LineStyle {
    pub id: Option<String>,
    pub color: String,
    pub color_mode: ColorMode,
    pub width: f64,
    pub attrs: HashMap<String, String>,
}

impl Default for LineStyle {
    fn default() -> LineStyle {
        LineStyle {
            id: None,
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            width: 1.0,
            attrs: HashMap::new(),
        }
    }
}

/// `kml:PolyStyle`, [12.16](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#927) in the
/// KML specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolyStyle {
    pub id: Option<String>,
    pub color: String,
    pub color_mode: ColorMode,
    pub fill: bool,
    pub outline: bool,
    pub attrs: HashMap<String, String>,
}

impl Default for PolyStyle {
    fn default() -> PolyStyle {
        PolyStyle {
            id: None,
            color: "ffffffff".to_string(),
            color_mode: ColorMode::default(),
            fill: true,
            outline: true,
            attrs: HashMap::new(),
        }
    }
}

/// `kml:listItemType`, [12.18](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#955) in the
/// KML specification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ListItemType {
    #[default]
    Check,
    CheckOffOnly,
    CheckHideChildren,
    RadioFolder,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListStyle {
    pub id: Option<String>,
    pub bg_color: String,
    pub max_snippet_lines: u32,
    pub list_item_type: ListItemType,
    pub attrs: HashMap<String, String>,
}

impl Default for ListStyle {
    fn default() -> ListStyle {
        ListStyle {
            id: None,
            bg_color: "ffffffff".to_string(),
            max_snippet_lines: 2,
            list_item_type: ListItemType::default(),
            attrs: HashMap::new(),
        }
    }
}
