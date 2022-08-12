use crate::types::Alias;
use std::collections::HashMap;

/// `kml:ResourceMap`, [10.13](https://docs.ogc.org/is/12-007r2/12-007r2.html#591) in the KML specification.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceMap {
    pub aliases: Option<Vec<Alias>>,
    pub attrs: HashMap<String, String>,
}
