//! # kml
//!
//! Rust support for reading and writing KML with a focus on conversion to [`geo-types`](https://github.com/georust/geo)
//! primitives.
//!
//! ## Examples
//!
//! ### Reading
//!
//! ```
//! use std::path::Path;
//! use kml::{Kml, KmlReader};
//!
//! let kml_str = r#"
//! <Polygon>
//!   <outerBoundaryIs>
//!     <LinearRing>
//!     <tessellate>1</tessellate>
//!     <coordinates>
//!         -1,2,0
//!         -1.5,3,0
//!         -1.5,2,0
//!         -1,2,0
//!     </coordinates>
//!     </LinearRing>
//!   </outerBoundaryIs>
//! </Polygon>
//! "#;
//!
//! // Parse from a string
//! let kml: Kml = kml_str.parse().unwrap();
//!
//! // Read from a file path
//! let kml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
//!     .join("tests")
//!     .join("fixtures")
//!     .join("polygon.kml");
//! let mut kml_reader = KmlReader::<_, f64>::from_path(kml_path).unwrap();
//! let kml_data = kml_reader.read().unwrap();
//!
//! // Read KMZ files with the `zip` feature or default features enabled
//! # #[cfg(feature = "zip")] {
//! let kmz_path = Path::new(env!("CARGO_MANIFEST_DIR"))
//!     .join("tests")
//!     .join("fixtures")
//!     .join("polygon.kmz");
//! let mut kmz_reader = KmlReader::<_, f64>::from_kmz_path(kmz_path).unwrap();
//! let kmz_data = kmz_reader.read().unwrap();
//! # }
//! ```
//!
//! ### Writing
//!
//! ```
//! use std::str;
//! use kml::{Kml, KmlWriter, types::Point};
//!
//! let kml = Kml::Point(Point::new(1., 1., None));
//!
//! let mut buf = Vec::new();
//! let mut writer = KmlWriter::from_writer(&mut buf);
//! writer.write(&kml).unwrap();
//! ```
//!
//! ### Conversion
//!
//! ```
//! # #[cfg(feature = "geo-types")] {
//! use geo_types::{self, GeometryCollection};
//! use kml::{Kml, types::Point};
//!
//! let kml_point = Point::new(1., 1., None);
//! // Convert into geo_types primitives
//! let geo_point = geo_types::Point::from(kml_point);
//! // Convert back into kml::types structs
//! let kml_point = Point::from(geo_point);
//!
//! let kml_folder_str = r#"
//! <Folder>
//!   <Point>
//!     <coordinates>1,1,1</coordinates>
//!     <altitudeMode>relativeToGround</altitudeMode>
//!   </Point>
//!   <LineString>
//!     <coordinates>1,1 2,1 3,1</coordinates>
//!     <altitudeMode>relativeToGround</altitudeMode>
//!   </LineString>
//! </Folder>"#;
//! let kml_folder: Kml<f64> = kml_folder_str.parse().unwrap();
//!
//! let geom_coll: GeometryCollection<f64> = kml_folder.try_into().unwrap();
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod types;

pub use crate::types::{Kml, KmlDocument, KmlVersion};

mod errors;
pub use crate::errors::Error;

pub mod reader;
pub use crate::reader::KmlReader;

pub mod writer;
pub use crate::writer::KmlWriter;

#[cfg(feature = "geo-types")]
pub mod conversion;

#[cfg(feature = "geo-types")]
#[allow(deprecated)]
pub use conversion::quick_collection;

#[cfg(feature = "zip")]
mod kmz_reader;

#[allow(unused_imports)]
#[cfg(feature = "zip")]
pub use kmz_reader::*;
