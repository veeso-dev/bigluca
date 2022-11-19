//! # Dubai papi configuration

use std::path::PathBuf;

/// Dubai papi generator configuration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DubaiPapiConfiguration {
    pub assets: Assets,
    pub gender_root: PathBuf,
}

/// Assets configuration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Assets {
    pub background: Background,
    pub car: Car,
    pub ear_pods: EarPods,
    pub eyes: Eyes,
    pub glasses: Glasses,
    pub hair_style: HairStyle,
    pub hat_color: HatColor,
    pub skin: Skin,
    pub top: Top,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Background {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Car {
    pub baguette: PathBuf,
    pub lambo: PathBuf,
    pub nikola: PathBuf,
    pub rolls_royal: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EarPods {
    pub black: PathBuf,
    pub white: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Eyes {
    pub black: PathBuf,
    pub blue: PathBuf,
    pub brown: PathBuf,
    pub green: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Glasses {
    pub eyeglasses: PathBuf,
    pub sunglasses: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HairStyle {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HatColor {
    pub black: PathBuf,
    pub cyan: PathBuf,
    pub green: PathBuf,
    pub red: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Skin {
    pub dark: PathBuf,
    pub olive: PathBuf,
    pub white: PathBuf,
    pub asian: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Top {
    pub black_jacket: PathBuf,
    pub blue_jacket: PathBuf,
    pub shirt: PathBuf,
    pub tank_top: PathBuf,
    pub black_t_shirt: PathBuf,
    pub orange_t_shirt: PathBuf,
    pub red_t_shirt: PathBuf,
    pub green_t_shirt: PathBuf,
    pub white_t_shirt: PathBuf,
}
