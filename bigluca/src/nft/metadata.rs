//! # Metadata

use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA256};

/// NFT Metadata
/// reference: <https://docs.opensea.io/docs/metadata-standards>
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    /// A human readable description of the item. Markdown is supported.
    pub description: String,
    /// This is the URL to the image of the item. Can be just about any type of image
    /// and can be IPFS URLs or paths.
    /// We recommend using a 350 x 350 image.
    image: String,
    /// Name of the item.
    pub name: String,
    /// These are the attributes for the item
    pub attributes: Vec<Attribute>,
}

/// Attributes related to this item
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

/// Trait to generate attribute parameters
pub trait AsAttribute {
    fn as_attribute(&self) -> Attribute;
}

impl Metadata {
    /// Instantiates a new Metadata item
    pub fn new(description: String, name: String, attributes: Vec<Attribute>) -> Self {
        Self {
            description,
            image: String::new(),
            name,
            attributes,
        }
    }

    /// Return NFT hash
    pub fn hash(&self) -> String {
        let mut digest_ctx = Context::new(&SHA256);
        // NOTE: attributes must be sorted for safety reasons, otherwise two nfts with same attributes would have different hash
        let mut sorted_attributes = self.attributes.clone();
        sorted_attributes.sort_by_key(|x| x.trait_type.clone());
        for attribute in sorted_attributes.iter() {
            attribute.hash(&mut digest_ctx);
        }
        let sha256 = digest_ctx.finish();
        HEXLOWER.encode(sha256.as_ref())
    }

    /// Return a string which contains prettified attributes
    pub fn pretty_attributes(&self) -> String {
        let mut s = String::default();
        for attribute in self.attributes.iter() {
            s.push_str(format!("{}: {} ", attribute.trait_type, attribute.value).as_str());
        }
        s
    }
}

impl Attribute {
    /// Instantiates a new Attribute item
    pub fn new(trait_type: impl ToString, value: impl ToString) -> Self {
        Self {
            trait_type: trait_type.to_string(),
            value: value.to_string(),
        }
    }

    /// Hash attribute
    fn hash(&self, ctx: &mut Context) {
        ctx.update(self.trait_type.as_bytes());
        ctx.update(self.value.as_bytes());
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn should_init_attributes() {
        let metadata = Metadata::new(
            String::from("test"),
            String::from("pippo"),
            vec![
                Attribute::new("collection", "Dubai Papi"),
                Attribute::new("HatColor", "Orange"),
                Attribute::new("Skin", "Dark"),
            ],
        );
        assert_eq!(metadata.description.as_str(), "test");
        assert_eq!(metadata.name.as_str(), "pippo");
        assert_eq!(metadata.attributes.len(), 3);
    }

    #[test]
    fn should_metadata_with_same_attributes_have_same_hash_even_with_unordered_attributes() {
        let metadata_a = Metadata::new(
            "test".to_string(),
            "Big Luca".to_string(),
            vec![
                Attribute::new("Collection", "Dubai Papi"),
                Attribute::new("HatColor", "Orange"),
                Attribute::new("Skin", "Dark"),
            ],
        );
        let metadata_b = Metadata::new(
            "test".to_string(),
            "Big Luca".to_string(),
            vec![
                Attribute::new("Collection", "Dubai Papi"),
                Attribute::new("Skin", "Dark"),
                Attribute::new("HatColor", "Orange"),
            ],
        );
        assert_eq!(metadata_a.hash(), metadata_b.hash());
    }

    #[test]
    fn should_metadata_with_different_attributes_have_different_hash() {
        let metadata_a = Metadata::new(
            "test".to_string(),
            "Big Luca".to_string(),
            vec![
                Attribute::new("collection", "Dubai Papi"),
                Attribute::new("HatColor", "Orange"),
                Attribute::new("Skin", "Dark"),
            ],
        );
        let metadata_b = Metadata::new(
            "test".to_string(),
            "Big Luca".to_string(),
            vec![
                Attribute::new("collection", "Dubai Papi"),
                Attribute::new("Skin", "Olive"),
                Attribute::new("HatColor", "Blue"),
            ],
        );
        assert_ne!(metadata_a.hash(), metadata_b.hash());
    }
}
