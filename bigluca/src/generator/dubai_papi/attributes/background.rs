use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Background {
    BurjAlArab,
    BurjKhalifa,
    Downtown,
    DubaiMarinaDay,
    DubaiMarinaNight,
    SkylineByNight,
}

impl AsAttribute for Background {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Background",
            match self {
                Self::BurjAlArab => "Burj Al Arab",
                Self::BurjKhalifa => "Burj Khalifa",
                Self::Downtown => "Downtown",
                Self::DubaiMarinaDay => "Dubai Marina by Day",
                Self::DubaiMarinaNight => "Dubai Marina by Night",
                Self::SkylineByNight => "Skyline by Night",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Background {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(match self {
            Self::BurjAlArab => &paths.assets.background.burj_al_arab,
            Self::BurjKhalifa => &paths.assets.background.burj_khalifa,
            Self::Downtown => &paths.assets.background.downtown,
            Self::DubaiMarinaDay => &paths.assets.background.dubai_marina_day,
            Self::DubaiMarinaNight => &paths.assets.background.dubai_marina_night,
            Self::SkylineByNight => &paths.assets.background.skyline_night,
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
            Background::all(),
            &[
                Background::BurjAlArab,
                Background::BurjKhalifa,
                Background::Downtown,
                Background::DubaiMarinaDay,
                Background::DubaiMarinaNight,
                Background::SkylineByNight,
            ]
        )
    }
}
