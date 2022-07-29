use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

/// `kml:Link`, [13.1](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#974) in the KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub href: Option<String>,
    pub refresh_mode: Option<RefreshMode>,
    pub refresh_interval: f64,
    pub view_refresh_mode: Option<ViewRefreshMode>,
    pub view_refresh_time: f64,
    pub view_bound_scale: f64,
    pub view_format: Option<String>,
    pub http_query: Option<String>,
    pub attrs: HashMap<String, String>,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            href: None,
            refresh_mode: None,
            refresh_interval: 4.0,
            view_refresh_mode: None,
            view_refresh_time: 4.0,
            view_bound_scale: 1.0,
            view_format: None,
            http_query: None,
            attrs: HashMap::new(),
        }
    }
}

/// `kml:Icon`, [13.1](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#974) in the KML specification.
#[derive(Clone, Debug, PartialEq)]
pub struct Icon {
    pub href: Option<String>,
    pub refresh_mode: Option<RefreshMode>,
    pub refresh_interval: f64,
    pub view_refresh_mode: Option<ViewRefreshMode>,
    pub view_refresh_time: f64,
    pub view_bound_scale: f64,
    pub view_format: Option<String>,
    pub http_query: Option<String>,
    pub attrs: HashMap<String, String>,
}

impl Default for Icon {
    fn default() -> Self {
        Self {
            href: None,
            refresh_mode: None,
            refresh_interval: 4.0,
            view_refresh_mode: None,
            view_refresh_time: 4.0,
            view_bound_scale: 1.0,
            view_format: None,
            http_query: None,
            attrs: HashMap::new(),
        }
    }
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
