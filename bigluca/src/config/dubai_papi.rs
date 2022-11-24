//! # Dubai papi configuration

use super::Validate;

use std::path::PathBuf;

/// Dubai papi generator configuration
#[derive(Debug, ValidateAllFields, Clone, PartialEq, Deserialize)]
pub struct DubaiPapiConfiguration {
    pub assets: Assets,
}

/// Assets configuration
#[derive(Debug, ValidateAllFields, Clone, PartialEq, Deserialize)]
pub struct Assets {
    pub background: Background,
    pub beard: Beard,
    pub car: Car,
    pub ear_pods: EarPods,
    pub eyes: Eyes,
    pub glasses: Glasses,
    pub hair_style: HairStyle,
    pub hat_color: HatColor,
    pub skin: Skin,
    pub top: Top,
}

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
pub struct Background {
    pub burj_al_arab: PathBuf,
    pub burj_khalifa: PathBuf,
    pub downtown: PathBuf,
    pub dubai_marina_day: PathBuf,
    pub dubai_marina_night: PathBuf,
    pub skyline_night: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
pub struct Beard {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CarColor {
    pub black: PathBuf,
    pub blue: PathBuf,
    pub gray: PathBuf,
    pub green: PathBuf,
    pub orange: PathBuf,
    pub pink: PathBuf,
    pub red: PathBuf,
    pub white: PathBuf,
    pub yellow: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Car {
    pub car_color: CarColor,
    pub baguette: PathBuf,
    pub lambo: PathBuf,
    pub nikola: PathBuf,
    pub rolls_royal: PathBuf,
}

impl Validate for Car {
    fn validate(&self) -> anyhow::Result<()> {
        let colors = &[
            &self.car_color.black,
            &self.car_color.blue,
            &self.car_color.gray,
            &self.car_color.green,
            &self.car_color.orange,
            &self.car_color.pink,
            &self.car_color.red,
            &self.car_color.white,
            &self.car_color.yellow,
        ];
        let cars = &[&self.baguette, &self.lambo, &self.nikola, &self.rolls_royal];
        for car in cars {
            for color in colors {
                let mut path = car.to_path_buf();
                path.push(color);
                if !path.exists() {
                    anyhow::bail!("car {} doesn't exist", path.display());
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
pub struct EarPods {
    pub black: PathBuf,
    pub white: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
pub struct Eyes {
    pub black: PathBuf,
    pub blue: PathBuf,
    pub brown: PathBuf,
    pub green: PathBuf,
}

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
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
pub struct HairStyle {
    pub hair_color: HairColor,
    pub bald: PathBuf,
    pub bob_cut: PathBuf,
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
        let hair_styles = &[&self.bob_cut];
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

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
pub struct HatColor {
    pub black: PathBuf,
    pub cyan: PathBuf,
    pub green: PathBuf,
    pub red: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Skin {
    pub root: PathBuf,
    pub male: PathBuf,
    pub female: PathBuf,
    pub dark: PathBuf,
    pub olive: PathBuf,
    pub white: PathBuf,
    pub asian: PathBuf,
}

impl Validate for Skin {
    fn validate(&self) -> anyhow::Result<()> {
        let skins = &[&self.dark, &self.olive, &self.white, &self.asian];
        let mut male_root = self.root.clone();
        male_root.extend(&self.male);
        let mut female_root = self.root.clone();
        female_root.extend(&self.female);
        // check skins
        for skin in skins {
            let mut male_path = male_root.to_path_buf();
            male_path.push(skin);
            let mut female_path = female_root.to_path_buf();
            female_path.push(skin);
            if !male_path.exists() {
                anyhow::bail!("{} male skin not found", male_path.display());
            }
            if !female_path.exists() {
                anyhow::bail!("{} female skin not found", female_path.display());
            }
        }

        Ok(())
    }
}

#[derive(Debug, ValidateAllPaths, Clone, PartialEq, Deserialize)]
pub struct Top {
    pub black_jacket: PathBuf,
    pub blue_jacket: PathBuf,
    pub shirt: PathBuf,
    pub tank_top: PathBuf,
    pub black_t_shirt: PathBuf,
    pub blue_t_shirt: PathBuf,
    pub orange_t_shirt: PathBuf,
    pub red_t_shirt: PathBuf,
    pub green_t_shirt: PathBuf,
    pub white_t_shirt: PathBuf,
}
