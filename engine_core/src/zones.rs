use std::fmt::Debug;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    Player,
    cards::{self, Card, CardInfo},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MainDeck {
    cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDeck {
    cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hand {
    pub cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug)]
pub struct Memory {
    cards: Vec<Card>,
    zone_info: ZoneInformation,
}

pub struct Graveyard {
    cards: Vec<Card>,
}

pub struct Field {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneInformation {
    visibility: Visibility,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    PUBLIC,
    PRIVATE,
}

pub trait Zone {}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PRIVATE,
            },
        }
    }

    pub fn add_card(&mut self, card_info: CardInfo) {
        self.cards.push(Card::from(card_info));
    }
}

impl MainDeck {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PRIVATE,
            },
        }
    }

    pub fn new_random(card_info: Vec<CardInfo>) -> Self {
        let mut deck = Self::new();

        let mut rng = rand::rng();

        deck.cards = (0..59)
            .into_iter()
            .map(|_| Card::from(card_info[rng.random_range(0..card_info.len())].clone()))
            .collect::<Vec<Card>>();

        deck
    }

    pub fn draw_from_top(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

impl MaterialDeck {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PRIVATE,
            },
        }
    }
}
