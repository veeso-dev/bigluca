//! # Dubai papi collection attributes
//!
//! This module defines the attributes for the collection

use crate::{config::DubaiPapiConfiguration, nft::Attribute as NftAttribute};

mod background;
mod beard;
mod car;
mod car_color;
mod ear_pods;
mod gender;
mod glasses;
mod hair_color;
mod hair_style;
mod hat_color;
mod skin;
mod top;

pub use background::Background;
pub use beard::Beard;
pub use car::Car;
pub use car_color::CarColor;
pub use ear_pods::EarPods;
pub use gender::Gender;
pub use glasses::Glasses;
pub use hair_color::HairColor;
pub use hair_style::HairStyle;
pub use hat_color::HatColor;
pub use skin::Skin;
pub use top::Top;
