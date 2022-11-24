//! # Collection

use crate::nft::{AsAttribute, Attribute};

use std::str::FromStr;

const DUBAI_PAPI: &str = "dubai-papi";

/// Defines collection name. Each collection should have this attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Collection {
    DubaiPapi,
}

impl FromStr for Collection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            DUBAI_PAPI => Ok(Self::DubaiPapi),
            collection => anyhow::bail!("unknown collection {}", collection),
        }
    }
}

impl ToString for Collection {
    fn to_string(&self) -> String {
        match self {
            Self::DubaiPapi => "Dubai Papi",
        }
        .to_string()
    }
}

impl AsAttribute for Collection {
    fn as_attribute(&self) -> Attribute {
        Attribute::new("Collection", self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_convert_str_to_collection() {
        assert_eq!(
            Collection::from_str("dubai-papi").unwrap(),
            Collection::DubaiPapi
        );
        assert!(Collection::from_str("test").is_err());
    }

    #[test]
    fn should_convert_collection_to_str() {
        assert_eq!(Collection::DubaiPapi.to_string().as_str(), "Dubai Papi");
    }

    #[test]
    fn should_convert_collection_to_attribute() {
        assert_eq!(
            Collection::DubaiPapi.as_attribute(),
            Attribute::new("Collection", "Dubai Papi")
        );
    }
}
