//! # Metadata

/// NFT Metadata
/// reference: <https://docs.opensea.io/docs/metadata-standards>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    /// A human readable description of the item. Markdown is supported.
    description: String,
    /// This is the URL to the image of the item. Can be just about any type of image
    /// and can be IPFS URLs or paths.
    /// We recommend using a 350 x 350 image.
    image: String,
    /// Name of the item.
    name: String,
    /// These are the attributes for the item
    attributes: Vec<Attribute>,
}

/// Attributes related to this item
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Attribute {
    trait_type: String,
    value: String,
}

/// Trait to generate attribute parameters
pub trait IntoAttribute {
    fn into(&self) -> Attribute;
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
}

impl Attribute {
    /// Instantiates a new Attribute item
    pub fn new(trait_type: impl ToString, value: impl ToString) -> Self {
        Self {
            trait_type: trait_type.to_string(),
            value: value.to_string(),
        }
    }
}
