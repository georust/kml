use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

/// Common model for `kml:LinkType`, [13.1](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#974) in the KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct LinkModel {
    pub href: String,
    pub refresh_mode: Option<RefreshMode>,
    pub refresh_interval: f64,
    pub view_refresh_mode: Option<ViewRefreshMode>,
    pub view_refresh_time: f64,
    pub view_bound_scale: f64,
    pub view_format: String,
    pub http_query: String,
    pub attrs: HashMap<String, String>,
}

impl Default for LinkModel {
    fn default() -> LinkModel {
        LinkModel {
            href: "".to_string(),
            refresh_mode: None,
            refresh_interval: 4.0,
            view_refresh_mode: None,
            view_refresh_time: 4.0,
            view_bound_scale: 1.0,
            view_format: "".to_string(),
            http_query: "".to_string(),
            attrs: HashMap::new(),
        }
    }
}

/// `kml:Link` and `kml:Icon`, [13.1](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#974) in the KML specification.
#[derive(Clone, Debug, PartialEq)]
pub enum LinkType {
    Icon(LinkModel),
    Link(LinkModel),
}

/// `kml:refreshModeEnumType`, [16.21](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#1239) in the KML specification.
#[derive(Clone, Debug, PartialEq)]
pub enum RefreshMode {
    OnChange,
    OnInterval,
    OnExpire,
}

impl Default for RefreshMode {
    fn default() -> RefreshMode {
        RefreshMode::OnChange
    }
}

impl Display for RefreshMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RefreshMode::OnChange => write!(f, "onChange"),
            RefreshMode::OnInterval => write!(f, "onInterval"),
            RefreshMode::OnExpire => write!(f, "onExpire"),
        }
    }
}

/// `kml:viewRefreshModeEnumType`, [16.27](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#1270) in the KML specification.
#[derive(Clone, Debug, PartialEq)]
pub enum ViewRefreshMode {
    Never,
    OnRequest,
    OnStop,
    OnRegion,
}

impl Default for ViewRefreshMode {
    fn default() -> ViewRefreshMode {
        ViewRefreshMode::Never
    }
}

impl Display for ViewRefreshMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ViewRefreshMode::Never => write!(f, "never"),
            ViewRefreshMode::OnRequest => write!(f, "onRequest"),
            ViewRefreshMode::OnStop => write!(f, "onStop"),
            ViewRefreshMode::OnRegion => write!(f, "onRegion"),
        }
    }
}
