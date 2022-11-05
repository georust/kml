use std::collections::HashMap;

/// `kml:SchemaData`, [9.5](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#155) in the KML specification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchemaData {
    pub data: Vec<SimpleData>,
    pub arrays: Vec<SimpleArrayData>,
    pub attrs: HashMap<String, String>,
}

/// `kml:SimpleData`, [9.6](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#167) in the KML specification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimpleData {
    pub name: String,
    pub value: String,
    pub attrs: HashMap<String, String>,
}

/// `kml:SimpleArrayData`, [9.7](https://docs.opengeospatial.org/is/12-007r2/12-007r2.html#177) in the KML specification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimpleArrayData {
    pub name: String,
    pub values: Vec<String>,
    pub attrs: HashMap<String, String>,
}
