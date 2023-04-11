use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::Error;

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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RefreshMode {
    #[default]
    OnChange,
    OnInterval,
    OnExpire,
}

impl FromStr for RefreshMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "onChange" => Ok(Self::OnChange),
            "onInterval" => Ok(Self::OnInterval),
            "onExpire" => Ok(Self::OnExpire),
            v => Err(Error::InvalidRefreshMode(v.to_string())),
        }
    }
}

impl fmt::Display for RefreshMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefreshMode::OnChange => write!(f, "onChange"),
            RefreshMode::OnInterval => write!(f, "onInterval"),
            RefreshMode::OnExpire => write!(f, "onExpire"),
        }
    }
}

/// `kml:viewRefreshModeEnumType`, [16.27](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#1270) in the KML specification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ViewRefreshMode {
    #[default]
    Never,
    OnRequest,
    OnStop,
    OnRegion,
}

impl FromStr for ViewRefreshMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "never" => Ok(Self::Never),
            "onRequest" => Ok(Self::OnRequest),
            "onStop" => Ok(Self::OnStop),
            "onRegion" => Ok(Self::OnRegion),
            v => Err(Error::InvalidViewRefreshMode(v.to_string())),
        }
    }
}

impl fmt::Display for ViewRefreshMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViewRefreshMode::Never => write!(f, "never"),
            ViewRefreshMode::OnRequest => write!(f, "onRequest"),
            ViewRefreshMode::OnStop => write!(f, "onStop"),
            ViewRefreshMode::OnRegion => write!(f, "onRegion"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_mode_from_str() {
        assert_eq!(
            RefreshMode::OnChange,
            RefreshMode::from_str("onChange").unwrap()
        );
        assert_eq!(
            RefreshMode::OnExpire,
            RefreshMode::from_str("onExpire").unwrap()
        );
        assert_eq!(
            RefreshMode::OnInterval,
            RefreshMode::from_str("onInterval").unwrap()
        );
    }

    #[test]
    fn test_view_refresh_mode_from_str() {
        assert_eq!(
            ViewRefreshMode::Never,
            ViewRefreshMode::from_str("never").unwrap()
        );
        assert_eq!(
            ViewRefreshMode::OnRegion,
            ViewRefreshMode::from_str("onRegion").unwrap()
        );
        assert_eq!(
            ViewRefreshMode::OnRequest,
            ViewRefreshMode::from_str("onRequest").unwrap()
        );
        assert_eq!(
            ViewRefreshMode::OnStop,
            ViewRefreshMode::from_str("onStop").unwrap()
        );
    }
}
