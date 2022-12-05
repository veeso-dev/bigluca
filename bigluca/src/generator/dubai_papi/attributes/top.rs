use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute, FromAttributes},
    render::{AsLayer, Layer},
};

const TRAIT_TYPE: &str = "Top";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Top {
    BlackJacket,
    BlueJacket,
    Shirt,
    CyanShirt,
    YellowShirt,
    TankTop,
    BlackTShirt,
    BlueTShirt,
    GreenTShirt,
    OrangeTShirt,
    RedTShirt,
    PinkTShirt,
    WhiteTShirt,
}

impl Top {
    pub fn male() -> &'static [Top] {
        &[
            Self::BlackJacket,
            Self::BlueJacket,
            Self::Shirt,
            Self::TankTop,
            Self::BlackTShirt,
            Self::BlueTShirt,
            Self::GreenTShirt,
            Self::OrangeTShirt,
            Self::RedTShirt,
            Self::WhiteTShirt,
        ]
    }

    pub fn female() -> &'static [Top] {
        &[
            Self::Shirt,
            Self::CyanShirt,
            Self::YellowShirt,
            Self::TankTop,
            Self::BlackTShirt,
            Self::BlueTShirt,
            Self::GreenTShirt,
            Self::OrangeTShirt,
            Self::RedTShirt,
            Self::PinkTShirt,
            Self::WhiteTShirt,
        ]
    }
}

impl AsAttribute for Top {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            TRAIT_TYPE,
            match self {
                Self::BlackJacket => "Black Jacket",
                Self::BlueJacket => "Blue Jacket",
                Self::Shirt => "Shirt",
                Self::CyanShirt => "Cyan Shirt",
                Self::YellowShirt => "Yellow Shirt",
                Self::TankTop => "Tank Top",
                Self::BlackTShirt => "Black T-Shirt",
                Self::BlueTShirt => "Blue T-Shirt",
                Self::GreenTShirt => "Green T-Shirt",
                Self::OrangeTShirt => "Orange T-Shirt",
                Self::RedTShirt => "Red T-Shirt",
                Self::PinkTShirt => "Pink T-Shirt",
                Self::WhiteTShirt => "White T-Shirt",
            },
        )
    }
}

impl FromAttributes for Top {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .and_then(|x| match x.value.as_str() {
                "Black Jacket" => Some(Self::BlackJacket),
                "Blue Jacket" => Some(Self::BlueJacket),
                "Shirt" => Some(Self::Shirt),
                "Cyan Shirt" => Some(Self::CyanShirt),
                "Yellow Shirt" => Some(Self::YellowShirt),
                "Tank Top" => Some(Self::TankTop),
                "Black T-Shirt" => Some(Self::BlackTShirt),
                "Blue T-Shirt" => Some(Self::BlueTShirt),
                "Green T-Shirt" => Some(Self::GreenTShirt),
                "Orange T-Shirt" => Some(Self::OrangeTShirt),
                "Red T-Shirt" => Some(Self::RedTShirt),
                "Pink T-Shirt" => Some(Self::PinkTShirt),
                "White T-Shirt" => Some(Self::WhiteTShirt),
                _ => None,
            })
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Top {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(match self {
            Self::BlackJacket => &paths.assets.top.black_jacket,
            Self::BlueJacket => &paths.assets.top.blue_jacket,
            Self::Shirt => &paths.assets.top.shirt,
            Self::CyanShirt => &paths.assets.top.cyan_shirt,
            Self::YellowShirt => &paths.assets.top.yellow_shirt,
            Self::TankTop => &paths.assets.top.tank_top,
            Self::BlackTShirt => &paths.assets.top.black_t_shirt,
            Self::BlueTShirt => &paths.assets.top.blue_t_shirt,
            Self::GreenTShirt => &paths.assets.top.green_t_shirt,
            Self::OrangeTShirt => &paths.assets.top.orange_t_shirt,
            Self::PinkTShirt => &paths.assets.top.pink_t_shirt,
            Self::RedTShirt => &paths.assets.top.red_t_shirt,
            Self::WhiteTShirt => &paths.assets.top.white_t_shirt,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            Top::male(),
            &[
                Top::BlackJacket,
                Top::BlueJacket,
                Top::Shirt,
                Top::TankTop,
                Top::BlackTShirt,
                Top::BlueTShirt,
                Top::GreenTShirt,
                Top::OrangeTShirt,
                Top::RedTShirt,
                Top::WhiteTShirt,
            ]
        );
        assert_eq!(
            Top::female(),
            &[
                Top::Shirt,
                Top::CyanShirt,
                Top::YellowShirt,
                Top::TankTop,
                Top::BlackTShirt,
                Top::BlueTShirt,
                Top::GreenTShirt,
                Top::OrangeTShirt,
                Top::RedTShirt,
                Top::PinkTShirt,
                Top::WhiteTShirt,
            ]
        )
    }

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Top::Shirt.as_attribute().trait_type, "Top");
    }
}
