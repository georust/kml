//! # kml
//!
//! Rust support for reading and writing KML with a focus on conversion to [`geo-types`](https://github.com/georust/geo)
//! primitives.
//!
//! ## Example
//!
//! ```
//! use kml::Kml;

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
//! let kml: Kml = kml_str.parse().unwrap();
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
pub use conversion::quick_collection;

#[cfg(feature = "zip")]
mod kmz_reader;

#[cfg(feature = "zip")]
pub use kmz_reader::*;
