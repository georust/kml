use std::str::FromStr;

#[derive(Clone)]
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clampToGround" => Ok(AltitudeMode::ClampToGround),
            "relativeToGround" => Ok(AltitudeMode::RelativeToGround),
            "absolute" => Ok(AltitudeMode::Absolute),
            _ => Err(()),
        }
    }
}
