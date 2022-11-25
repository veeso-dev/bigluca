use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

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
            "Top",
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
}
