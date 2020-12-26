//! # kml
//!
//! `kml` provides basic KML support for Rust with conversions to `geo-types`
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

pub mod types;

pub use crate::types::{Kml, KmlDocument, KmlVersion};

mod errors;
pub use crate::errors::Error;

pub mod reader;
pub use crate::reader::KmlReader;

#[cfg(feature = "geo-types")]
pub mod conversion;

#[cfg(feature = "geo-types")]
pub use conversion::quick_collection;
