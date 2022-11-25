//! # Dubai papi configuration

use super::Validate;

use std::path::PathBuf;

/// Dubai papi generator configuration
#[derive(Debug, ValidateAllFields, Clone, Eq, PartialEq, Deserialize)]
pub struct DubaiPapiConfiguration {
    pub assets: Assets,
}

/// Assets configuration
#[derive(Debug, ValidateAllFields, Clone, Eq, PartialEq, Deserialize)]
pub struct Assets {
    pub background: Background,
    pub beard: Beard,
    pub head_phones: HeadPhones,
    pub eyes: Eyes,
    pub glasses: Glasses,
    pub hair_style: HairStyle,
    pub hat: Hat,
    pub mood: Mood,
    pub skin: Skin,
    pub top: Top,
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Background {
    pub burj_al_arab: PathBuf,
    pub burj_khalifa: PathBuf,
    pub downtown: PathBuf,
    pub dubai_marina_day: PathBuf,
    pub dubai_marina_night: PathBuf,
    pub skyline_night: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Beard {
    pub hair_color: HairColor,
    pub chevron: PathBuf,
    pub chin_strip: PathBuf,
    pub circle_beard: PathBuf,
    pub royale_beard: PathBuf,
    pub short_boxed_beard: PathBuf,
    pub three_day_stubble_beard: PathBuf,
}

impl Validate for Beard {
    fn validate(&self) -> anyhow::Result<()> {
        let colors = &[
            &self.hair_color.black,
            &self.hair_color.brown,
            &self.hair_color.blonde,
            &self.hair_color.blue,
            &self.hair_color.green,
            &self.hair_color.pink,
            &self.hair_color.red,
        ];
        let beard_styles = &[
            &self.chevron,
            &self.chin_strip,
            &self.circle_beard,
            &self.royale_beard,
            &self.short_boxed_beard,
            &self.three_day_stubble_beard,
        ];
        for style in beard_styles {
            for color in colors {
                let mut path = style.to_path_buf();
                path.push(color);
                if !path.exists() {
                    anyhow::bail!("{} beard style not found", path.display());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Eyes {
    pub black: PathBuf,
    pub blue: PathBuf,
    pub brown: PathBuf,
    pub green: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Glasses {
    pub eyeglasses: PathBuf,
    pub sunglasses: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct HairColor {
    pub black: PathBuf,
    pub brown: PathBuf,
    pub blonde: PathBuf,
    pub blue: PathBuf,
    pub green: PathBuf,
    pub pink: PathBuf,
    pub red: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct HairStyle {
    pub hair_color: HairColor,
    pub bob: PathBuf,
    pub curly: PathBuf,
    pub long: PathBuf,
    pub pony_tail: PathBuf,
    pub bald: PathBuf,
    pub bun: PathBuf,
    pub ivy_league: PathBuf,
    pub pixie: PathBuf,
    pub taper: PathBuf,
}

impl Validate for HairStyle {
    fn validate(&self) -> anyhow::Result<()> {
        if !self.bald.exists() {
            anyhow::bail!("bald path {} doesn't exist", self.bald.display());
        }
        let colors = &[
            &self.hair_color.black,
            &self.hair_color.brown,
            &self.hair_color.blonde,
            &self.hair_color.blue,
            &self.hair_color.green,
            &self.hair_color.pink,
            &self.hair_color.red,
        ];
        let hair_styles = &[
            &self.bob,
            &self.curly,
            &self.long,
            &self.pony_tail,
            &self.bun,
            &self.ivy_league,
            &self.pixie,
            &self.taper,
        ];
        for style in hair_styles {
            for color in colors {
                let mut path = style.to_path_buf();
                path.push(color);
                if !path.exists() {
                    anyhow::bail!("{} hair style not found", path.display());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Hat {
    pub black: PathBuf,
    pub cyan: PathBuf,
    pub green: PathBuf,
    pub pink: PathBuf,
    pub red: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct HeadPhones {
    pub ear_pods_black: PathBuf,
    pub ear_pods_white: PathBuf,
    pub head_phones_black: PathBuf,
    pub head_phones_white: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Mood {
    pub angry: PathBuf,
    pub happy: PathBuf,
    pub neutral: PathBuf,
    pub sad: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Skin {
    pub dark: PathBuf,
    pub olive: PathBuf,
    pub white: PathBuf,
    pub asian: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, Eq, PartialEq, Deserialize)]
pub struct Top {
    pub black_jacket: PathBuf,
    pub blue_jacket: PathBuf,
    pub shirt: PathBuf,
    pub cyan_shirt: PathBuf,
    pub yellow_shirt: PathBuf,
    pub tank_top: PathBuf,
    pub black_t_shirt: PathBuf,
    pub blue_t_shirt: PathBuf,
    pub orange_t_shirt: PathBuf,
    pub red_t_shirt: PathBuf,
    pub green_t_shirt: PathBuf,
    pub pink_t_shirt: PathBuf,
    pub white_t_shirt: PathBuf,
}
