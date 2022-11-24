use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum EarPods {
    Black,
    White,
}

impl AsAttribute for EarPods {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Ear Pods",
            match self {
                Self::Black => "Black",
                Self::White => "White",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for EarPods {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(
            match self {
                Self::Black => &paths.assets.ear_pods.black,
                Self::White => &paths.assets.ear_pods.white,
            },
            128,
            64,
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(EarPods::all(), &[EarPods::Black, EarPods::White])
    }
}
