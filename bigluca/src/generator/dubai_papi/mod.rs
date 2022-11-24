//! # Dubai papi collection generator
//!
//! Generator for the Dubai papi collection

mod attributes;
mod description;

use self::attributes::{
    Background, Beard, Eyes, Gender, Glasses, HairColor, HairStyle, HatColor, HeadPhones, Skin, Top,
};

use super::GenerateNft;
use crate::{
    config::DubaiPapiConfiguration,
    database::{names, nft::NftDatabase},
    generator::dubai_papi::{attributes::Mood, description::DescriptionGenerator},
    nft::{AsAttribute, Attribute as NftAttribute, Metadata, Nft},
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
            let head_phones = random.choice_or_none(HeadPhones::all(), 30).cloned();
            debug!("chosen ear pods: {:?}", head_phones);
            let eyes = *random.choice(Eyes::all());
            debug!("chosen eyes: {:?}", eyes);
            let glasses = random.choice_or_none(Glasses::all(), 25).cloned();
            debug!("chosen glasses: {:?}", glasses);
            let hair_style = *random.choice(match gender {
                Gender::Female => HairStyle::female(),
                Gender::Male => HairStyle::male(),
            });
            debug!("chosen hair style: {:?}", hair_style);
            let hair_color = *random.choice(HairColor::all());
            debug!("chosen hair color: {:?}", hair_color);
            let hat_color = random.choice_or_none(HatColor::all(), 20).cloned();
            debug!("chosen hat color: {:?}", hat_color);
            let mood = *random.choice(Mood::all());
            debug!("chosen mood: {:?}", mood);
            let skin = *random.choice(Skin::all());
            debug!("chosen skin: {:?}", skin);
            let top = *random.choice(Top::all());
            debug!("chosen top: {:?}", top);

            // make attributes
            let attributes: Vec<NftAttribute> = vec![
                Some(background.as_attribute()),
                beard.map(|x| x.as_attribute()),
                head_phones.map(|x| x.as_attribute()),
                Some(eyes.as_attribute()),
                Some(gender.as_attribute()),
                glasses.map(|x| x.as_attribute()),
                Some(hair_color.as_attribute()),
                Some(hair_style.as_attribute()),
                hat_color.map(|x| x.as_attribute()),
                Some(mood.as_attribute()),
                Some(skin.as_attribute()),
                Some(top.as_attribute()),
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
            let description = DescriptionGenerator::generate(&mut random, gender, &name);
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
