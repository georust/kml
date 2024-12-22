# kml

[![crates.io](https://img.shields.io/crates/v/kml.svg)](https://crates.io/crates/kml)
[![Build status](https://github.com/georust/kml/workflows/CI/badge.svg)](https://github.com/georust/kml/actions?query=workflow%3ACI)
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
let mut kml_reader = KmlReader::<_, f64>::from_path(kml_path).unwrap();
let kml_data = kml_reader.read().unwrap();

// Read KMZ files with the `zip` feature or default features enabled
let kmz_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("fixtures")
    .join("polygon.kmz");
let mut kmz_reader = KmlReader::<_, f64>::from_kmz_path(kmz_path).unwrap();
let kmz_data = kmz_reader.read().unwrap();
```

### Writing

```rust
use kml::{Kml, KmlWriter, types::Point};

let kml = Kml::Point(Point::new(1., 1., None));

let mut buf = Vec::new();
let mut writer = KmlWriter::from_writer(&mut buf);
writer.write(&kml).unwrap();
```

### Conversion

```rust
use geo_types::{self, GeometryCollection};
use kml::{Kml, types::Point};

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

let geom_coll: GeometryCollection<f64> = kml_folder.try_into().unwrap();
```

## Code of Conduct

All contributors are expected to follow the [GeoRust Code of Conduct](https://github.com/georust/.github/blob/main/CODE_OF_CONDUCT.md)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
