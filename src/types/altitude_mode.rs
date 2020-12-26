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
