use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HeadPhones {
    EarPodsBlack,
    EarPodsWhite,
    HeadPhonesBlack,
    HeadPhonesWhite,
}

impl AsAttribute for HeadPhones {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Ear Pods",
            match self {
                Self::EarPodsBlack => "Black Ear Pods",
                Self::EarPodsWhite => "White Ear Pods",
                Self::HeadPhonesBlack => "Black Head Phones",
                Self::HeadPhonesWhite => "White Head Phones",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for HeadPhones {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(
            match self {
                Self::EarPodsBlack => &paths.assets.head_phones.ear_pods_black,
                Self::EarPodsWhite => &paths.assets.head_phones.ear_pods_white,
                Self::HeadPhonesBlack => &paths.assets.head_phones.head_phones_black,
                Self::HeadPhonesWhite => &paths.assets.head_phones.head_phones_white,
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
            HeadPhones::all(),
            &[
                HeadPhones::EarPodsBlack,
                HeadPhones::EarPodsWhite,
                HeadPhones::HeadPhonesBlack,
                HeadPhones::HeadPhonesWhite
            ]
        )
    }
}
