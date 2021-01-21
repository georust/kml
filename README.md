# kml

[![crates.io](https://img.shields.io/crates/v/kml.svg)](https://crates.io/crates/kml)
[![Build status](https://github.com/pjsier/kml/workflows/CI/badge.svg)](https://github.com/pjsier/kml/actions?query=workflow%3ACI)
[![Documentation](https://docs.rs/kml/badge.svg)](https://docs.rs/kml)

Rust support for reading and writing KML with a focus on conversion to [`geo-types`](https://github.com/georust/geo) primitives.

## Examples

### Reading

```rust
use std::path::Path;
use kml::{Kml, KmlReader};

let kml_str = r#"
<Polygon>
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
</Polygon>
"#;

// Parse from a string
let kml: Kml = kml_str.parse().unwrap();

// Read from a file path
let kml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("fixtures")
    .join("polygon.kml");
let mut kml_reader = KmlReader::<_, f64>::from_file(kml_path).unwrap();
let kml_data = kml_reader.parse().unwrap();

// Read KMZ files with the `zip` feature or default features enabled
let kmz_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("fixtures")
    .join("polygon.kmz");
let mut kmz_reader = KmlReader::<_, f64>::from_kmz_file(kmz_path).unwrap();
let kmz_data = kmz_reader.parse().unwrap();
```

### Writing

```rust
use std::str;
use quick_xml;
use kml::{Kml, KmlWriter, types::{AltitudeMode, Coord, Point}};

let kml = Kml::Point(Point {
    coord: Coord {
        x: 1.,
        y: 1.,
        z: Some(1.),
    },
    ..Default::default()
});

let mut buf = Vec::new();
let mut writer = KmlWriter::new(quick_xml::Writer::new(&mut buf));
writer.write(&kml).unwrap();
```

### Conversion

```rust
use geo_types::{self, GeometryCollection};
use kml::{quick_collection, Kml, types::Point};

let kml_point = Point::new(1., 1., None);
// Convert into geo_types primitives
let geo_point = geo_types::Point::from(kml_point);
// Convert back into kml::types structs
let kml_point = Point::from(geo_point);

let kml_folder_str = r#"
<Folder>
  <Point>
    <coordinates>1,1,1</coordinates>
    <altitudeMode>relativeToGround</altitudeMode>
  </Point>
  <LineString>
    <coordinates>1,1 2,1 3,1</coordinates>
    <altitudeMode>relativeToGround</altitudeMode>
  </LineString>
</Folder>"#;
let kml_folder: Kml<f64> = kml_folder_str.parse().unwrap();

// Use the quick_collection helper to convert Kml to a geo_types::GeometryCollection
let geom_coll: GeometryCollection<f64> = quick_collection(kml_folder).unwrap();
```
