use std::path::PathBuf;

use super::CarColor;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{Attribute, IntoAttribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Car {
    Baguette,
    Lambo,
    Nikola,
    RollsRoyal,
}

impl Car {
    fn path(&self, config: &DubaiPapiConfiguration, color: CarColor) -> PathBuf {
        let mut car_path = match self {
            Self::Baguette => &config.assets.car.baguette,
            Self::Lambo => &config.assets.car.lambo,
            Self::Nikola => &config.assets.car.nikola,
            Self::RollsRoyal => &config.assets.car.rolls_royal,
        }
        .to_path_buf();
        car_path.push(match color {
            CarColor::Black => &config.assets.car.car_color.black,
            CarColor::Blue => &config.assets.car.car_color.blue,
            CarColor::Gray => &config.assets.car.car_color.gray,
            CarColor::Green => &config.assets.car.car_color.green,
            CarColor::Orange => &config.assets.car.car_color.orange,
            CarColor::Pink => &config.assets.car.car_color.pink,
            CarColor::Red => &config.assets.car.car_color.red,
            CarColor::White => &config.assets.car.car_color.white,
            CarColor::Yellow => &config.assets.car.car_color.yellow,
        });

        car_path
    }
}

impl IntoAttribute for Car {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Car",
            match self {
                Self::Baguette => "Baguette",
                Self::Lambo => "Lambo",
                Self::Nikola => "Nikola",
                Self::RollsRoyal => "RollsRoyal",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, CarColor> for Car {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, states: CarColor) -> anyhow::Result<Layer> {
        Layer::from_path(&self.path(paths, states), 24, 32)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            Car::all(),
            &[Car::Baguette, Car::Lambo, Car::Nikola, Car::RollsRoyal,]
        )
    }
}
