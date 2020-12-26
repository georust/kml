# kml

[![crates.io](https://img.shields.io/crates/v/kml.svg)](https://crates.io/crates/kml)
[![Build status](https://github.com/pjsier/kml/workflows/CI/badge.svg)](https://github.com/pjsier/kml/actions?query=workflow%3ACI)

[Documentation](https://docs.rs/kml/)

KML support for Rust with a focus on conversion to `geo-types`. Currently only partial read support is implemented.

## Examples

```rust
use kml::Kml;

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

let kml: Kml = kml_str.parse().unwrap();
```
