//! # Description

use crate::utils::random::Random;

use super::attributes::Gender;

pub struct DescriptionGenerator;

impl DescriptionGenerator {
    pub fn generate(random: &mut Random, gender: Gender, name: &str) -> String {
        let (subject, personal, possessive) = Self::pronouns(gender);
        // formulae: {name} started {his/her} activity of {START_ACTIVITY} and has become {CAPITAL} {ACTION} {OBJECTS}.
        // {he/she} currently lives in Dubai. In the weekend you can find {him/her} {hobby}.
        let action = *random.choice(Action::all());
        format!(
            "{} started {} activity of {} and has become {} {} {}. {} currently lives in Dubai. In the weekend you can find {} {}.",
            name,
            possessive,
            random.choice(START_ACTIVITY),
            random.choice(CAPITAL),
            action.to_string(),
            random.choice(action.objects()),
            subject,
            personal,
            random.choice(HOBBY),
        )
    }

    fn pronouns(gender: Gender) -> (&'static str, &'static str, &'static str) {
        match gender {
            Gender::Female => ("she", "her", "her"),
            Gender::Male => ("he", "him", "his"),
        }
    }
}

const START_ACTIVITY: &[&str] = &["trading", "crypto-trading", "dropshipping"];
const CAPITAL: &[&str] = &["millionaire", "rich", "billionaire"];
const HOBBY: &[&str] = &[
    "at the club",
    "playing golf",
    "on a yacht",
    "at the gym",
    "playing poker",
    "working on the next DOGE",
    "counting money",
];

#[derive(AllVariants, Copy, Clone)]
enum Action {
    Selling,
    Trading,
    Advertising,
    Publishing,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Self::Selling => "selling",
            Self::Trading => "trading",
            Self::Advertising => "advertising",
            Self::Publishing => "publishing",
        }
        .to_string()
    }
}

impl Action {
    fn objects(&self) -> &[&'static str] {
        match self {
            Self::Selling => &[
                "paintings",
                "action figures",
                "candles",
                "courses",
                "nsfw contents",
                "Team Fortress hats",
                "CS weapons",
                "real estate",
                "cheap drop shipping items",
                "NFTs",
            ],
            Self::Advertising => &[
                "crypto projects",
                "bet platforms",
                "cosmetics",
                "bags",
                "NortonVPN",
                "low budget mobile games",
            ],
            Self::Publishing => &[
                "books on investment strategies",
                "tons of courses about online marketing",
            ],
            Self::Trading => &[
                "SOL",
                "cryptocurrencies",
                "NFTs",
                "stocks",
                "ETH",
                "Bitcoins",
                "DOGE",
                "TAMA",
                "SHIB",
                "ADA",
            ],
        }
    }
}
