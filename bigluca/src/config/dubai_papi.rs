//! # Dubai papi configuration

use super::Validate;

use std::path::PathBuf;

/// Dubai papi generator configuration
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DubaiPapiConfiguration {
    pub assets: Assets,
    pub gender_root: PathBuf,
}

/// Assets configuration
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Assets {
    pub background: Background,
    pub beard: Beard,
    pub car: Car,
    pub ear_pods: EarPods,
    pub eyes: Eyes,
    pub glasses: Glasses,
    pub hair_color: HairColor,
    pub hair_style: HairStyle,
    pub hat_color: HatColor,
    pub skin: Skin,
    pub top: Top,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Background {
    pub burj_al_arab: PathBuf,
    pub burj_khalifa: PathBuf,
    pub downtown: PathBuf,
    pub dubai_marina_day: PathBuf,
    pub dubai_marina_night: PathBuf,
    pub skyline_night: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Beard {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Car {
    pub baguette: PathBuf,
    pub lambo: PathBuf,
    pub nikola: PathBuf,
    pub rolls_royal: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EarPods {
    pub black: PathBuf,
    pub white: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Eyes {
    pub black: PathBuf,
    pub blue: PathBuf,
    pub brown: PathBuf,
    pub green: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Glasses {
    pub eyeglasses: PathBuf,
    pub sunglasses: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HairColor {
    pub black: PathBuf,
    pub brown: PathBuf,
    pub blonde: PathBuf,
    pub blue: PathBuf,
    pub green: PathBuf,
    pub pink: PathBuf,
    pub red: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HairStyle {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HatColor {
    pub black: PathBuf,
    pub cyan: PathBuf,
    pub green: PathBuf,
    pub red: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Skin {
    pub dark: PathBuf,
    pub olive: PathBuf,
    pub white: PathBuf,
    pub asian: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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