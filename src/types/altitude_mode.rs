use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

// reference docs TODO:
#[derive(Copy, Clone, Debug, PartialEq)]
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
            _ => Err(Error::PlaceholderError),
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
