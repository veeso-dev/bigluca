use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute, FromAttributes},
    render::{AsLayer, Layer},
};

const TRAIT_TYPE: &str = "Background";

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
            TRAIT_TYPE,
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

impl FromAttributes for Background {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .map(|x| match x.value.as_str() {
                "Burj Al Arab" => Some(Self::BurjAlArab),
                "Burj Khalifa" => Some(Self::BurjKhalifa),
                "Downtown" => Some(Self::Downtown),
                "Dubai Marina by Day" => Some(Self::DubaiMarinaDay),
                "Dubai Marina by Night" => Some(Self::DubaiMarinaNight),
                "Skyline by Night" => Some(Self::SkylineByNight),
                _ => None,
            })
            .flatten()
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

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(
            &Background::BurjAlArab.as_attribute().trait_type,
            "Background"
        );
    }
}
