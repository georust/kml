use std::collections::HashMap;

/// `kml:Alias`, [10.14](https://docs.ogc.org/is/12-007r2/12-007r2.html#598) in the KML specification.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Alias {
    pub target_href: Option<String>,
    pub source_href: Option<String>,
    pub attrs: HashMap<String, String>,
}
