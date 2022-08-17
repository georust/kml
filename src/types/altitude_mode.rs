use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

/// `kml:altitudeMode`, [9.20](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#322) in the
/// KML specification
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AltitudeMode {
    ClampToGround,
    RelativeToGround,
    Absolute,
}

impl Default for AltitudeMode {
    fn default() -> AltitudeMode {
        AltitudeMode::ClampToGround
    }
}

impl FromStr for AltitudeMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clampToGround" => Ok(Self::ClampToGround),
            "relativeToGround" => Ok(Self::RelativeToGround),
            "absolute" => Ok(Self::Absolute),
            v => Err(Error::InvalidAltitudeMode(v.to_string())),
        }
    }
}

impl fmt::Display for AltitudeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ClampToGround => "clampToGround",
                Self::RelativeToGround => "relativeToGround",
                Self::Absolute => "absolute",
            }
        )
    }
}
