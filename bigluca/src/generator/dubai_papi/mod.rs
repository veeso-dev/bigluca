//! # Dubai papi collection generator
//!
//! Generator for the Dubai papi collection

mod attributes;
mod description;

use self::attributes::{
    Background, Beard, Car, CarColor, EarPods, Eyes, Gender, Glasses, HairColor, HairStyle,
    HatColor, Skin, Top,
};

use super::GenerateNft;
use crate::{
    config::DubaiPapiConfiguration,
    database::{names, nft::NftDatabase},
    generator::dubai_papi::description::DescriptionGenerator,
    nft::{Attribute as NftAttribute, IntoAttribute, Metadata, Nft},
    render::{AsLayer, Layer, RenderEngine},
    utils::{collisions::try_for, random::Random},
};

const MAX_MINT: usize = 1000;
/// Dubai papi collection generator
pub struct DubaiPapi<'a> {
    config: &'a DubaiPapiConfiguration,
    database: &'a mut NftDatabase,
}

impl<'a> DubaiPapi<'a> {
    /// Instantiates new DubaiPapi from configuration
    pub fn new(config: &'a DubaiPapiConfiguration, database: &'a mut NftDatabase) -> Self {
        Self { config, database }
    }

    /// Get cryptopapi name
    fn get_name(&self, random: &mut Random, gender: Gender) -> anyhow::Result<String> {
        try_for(
            || {
                let name = format!(
                    "{} {}",
                    random.choice(names::NAME_ATTRIBUTES),
                    random.choice(match gender {
                        Gender::Female => names::FEMALE_NAMES,
                        Gender::Male => names::MALE_NAMES,
                    })
                );
                if self.database.names.contains(&name) {
                    None
                } else {
                    Some(name)
                }
            },
            128,
        )
    }
}

impl<'a> GenerateNft for DubaiPapi<'a> {
    fn generate_nft(self) -> anyhow::Result<Nft> {
        if self.database.dubai_papi_hash.len() >= MAX_MINT {
            anyhow::bail!("cannot mint NFT: MAX MINT ({}) reached", MAX_MINT);
        }
        info!(
            "miting NFT #{}/{}",
            self.database.dubai_papi_hash.len() + 1,
            MAX_MINT
        );
        for _ in 0..64 {
            let mut random = Random::default();
            // select attributes
            let gender = *random.choice(Gender::all());
            debug!("chosen gender: {:?}", gender);
            let background = *random.choice(Background::all());
            debug!("chosen background: {:?}", background);
            let beard = if gender == Gender::Male {
                random.choice_or_none(Beard::all(), 80).cloned()
            } else {
                None
            };
            debug!("chosen beard: {:?}", beard);
            let car = random.choice_or_none(Car::all(), 50).cloned();
            debug!("chosen car: {:?}", car);
            let car_color = if car.is_some() {
                Some(*random.choice(CarColor::all()))
            } else {
                None
            };
            debug!("chosen car color: {:?}", car_color);
            let ear_pods = random.choice_or_none(EarPods::all(), 30).cloned();
            debug!("chosen ear pods: {:?}", ear_pods);
            let eyes = *random.choice(Eyes::all());
            debug!("chosen eyes: {:?}", eyes);
            let glasses = random.choice_or_none(Glasses::all(), 25).cloned();
            debug!("chosen glasses: {:?}", glasses);
            let hair_style = *random.choice(match gender {
                Gender::Male => &[HairStyle::Bald],
                Gender::Female => &[HairStyle::BobCut],
            });
            debug!("chosen hair style: {:?}", hair_style);
            let hair_color = *random.choice(HairColor::all());
            debug!("chosen hair color: {:?}", hair_color);
            let hat_color = random.choice_or_none(HatColor::all(), 20).cloned();
            debug!("chosen hat color: {:?}", hat_color);
            let skin = *random.choice(Skin::all());
            debug!("chosen skin: {:?}", skin);
            let top = *random.choice(Top::all());
            debug!("chosen top: {:?}", top);

            // make attributes
            let attributes: Vec<NftAttribute> = vec![
                Some(background.into_attribute()),
                beard.map(|x| x.into_attribute()),
                car.map(|x| x.into_attribute()),
                car_color.map(|x| x.into_attribute()),
                ear_pods.map(|x| x.into_attribute()),
                Some(eyes.into_attribute()),
                Some(gender.into_attribute()),
                glasses.map(|x| x.into_attribute()),
                Some(hair_color.into_attribute()),
                Some(hair_style.into_attribute()),
                hat_color.map(|x| x.into_attribute()),
                Some(skin.into_attribute()),
                Some(top.into_attribute()),
            ]
            .into_iter()
            .flatten()
            .collect();

            // get name
            let name = self.get_name(&mut random, gender)?;
            debug!("chosen name: {}", name);
            if self.database.names.contains(&name) {
                error!("collision detected with name: {}", name);
                continue;
            }
            let description = DescriptionGenerator::generate(&mut random, gender, &name, car);
            debug!("chosen description: {}", description);
            // make metadata
            let metadata = Metadata::new(description, name, attributes);
            let metadata_hash = metadata.hash();
            debug!("generated metadata with hash: {}", metadata_hash);
            if self.database.dubai_papi_hash.contains(&metadata_hash) {
                error!("collision detected with metadata hash: {}", metadata_hash);
                debug!("colliding metadata: {:?}", metadata);
                continue;
            }

            let layers: Vec<Layer> = vec![background.as_layer(self.config, ())?]
                .into_iter()
                .collect();
            let image = RenderEngine::render(350, 350, layers)?;
            // push changes to database
            self.database.names.push(metadata.name.clone());
            self.database.dubai_papi_hash.push(metadata_hash);

            return Ok(Nft::new(
                self.database.dubai_papi_hash.len(),
                image,
                metadata,
            ));
        }

        anyhow::bail!("failed to get a unique NFT after 64 attempts")
    }
}
