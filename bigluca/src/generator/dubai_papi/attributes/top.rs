use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Top {
    BlackJacket,
    BlueJacket,
    Shirt,
    TankTop,
    BlackTShirt,
    BlueTShirt,
    GreenTShirt,
    OrangeTShirt,
    RedTShirt,
    WhiteTShirt,
}

impl AsAttribute for Top {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Top",
            match self {
                Self::BlackJacket => "Black Jacket",
                Self::BlueJacket => "Blue Jacket",
                Self::Shirt => "Shirt",
                Self::TankTop => "Tank Top",
                Self::BlackTShirt => "Black T-Shirt",
                Self::BlueTShirt => "Blue T-Shirt",
                Self::GreenTShirt => "Green T-Shirt",
                Self::OrangeTShirt => "Orange T-Shirt",
                Self::RedTShirt => "Red T-Shirt",
                Self::WhiteTShirt => "White T-Shirt",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Top {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(
            match self {
                Self::BlackJacket => &paths.assets.top.black_jacket,
                Self::BlueJacket => &paths.assets.top.blue_jacket,
                Self::Shirt => &paths.assets.top.shirt,
                Self::TankTop => &paths.assets.top.tank_top,
                Self::BlackTShirt => &paths.assets.top.black_t_shirt,
                Self::BlueTShirt => &paths.assets.top.blue_t_shirt,
                Self::GreenTShirt => &paths.assets.top.green_t_shirt,
                Self::OrangeTShirt => &paths.assets.top.orange_t_shirt,
                Self::RedTShirt => &paths.assets.top.red_t_shirt,
                Self::WhiteTShirt => &paths.assets.top.white_t_shirt,
            },
            96,
            280,
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            Top::all(),
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
        )
    }
}
