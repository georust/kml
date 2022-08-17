use core::fmt;
use std::str::FromStr;

use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
    pub xunits: Units,
    pub yunits: Units,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 1.,
            y: 1.,
            xunits: Units::default(),
            yunits: Units::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Units {
    Fraction,
    Pixels,
    InsetPixels,
}

impl Default for Units {
    fn default() -> Self {
        Self::Fraction
    }
}

impl FromStr for Units {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fraction" => Ok(Self::Fraction),
            "pixels" => Ok(Self::Pixels),
            "insetPixels" => Ok(Self::InsetPixels),
            v => Err(Error::InvalidUnits(v.to_string())),
        }
    }
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fraction => "fraction",
                Self::Pixels => "pixels",
                Self::InsetPixels => "insetPixels",
            }
        )
    }
}
