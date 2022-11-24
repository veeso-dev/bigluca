//! # Mood
use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Angry,
    Happy,
    Neutral,
    Sad,
}

impl AsAttribute for Mood {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Glasses",
            match self {
                Self::Angry => "Angry",
                Self::Happy => "Happy",
                Self::Neutral => "Neutral",
                Self::Sad => "Sad",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Mood {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(
            match self {
                Self::Angry => &paths.assets.mood.angry,
                Self::Happy => &paths.assets.mood.happy,
                Self::Neutral => &paths.assets.mood.neutral,
                Self::Sad => &paths.assets.mood.sad,
            },
            0,
            0,
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
            Mood::all(),
            &[Mood::Angry, Mood::Happy, Mood::Neutral, Mood::Sad,]
        )
    }
}
