use core::fmt;
use std::fmt::Debug;

use rand::Rng;
use serde::{Deserialize, Serialize, de::value::Error};

use crate::{
    Player,
    cards::{self, Card, CardInfo},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MainDeck {
    cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MaterialDeck {
    cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Memory {
    pub cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Graveyard {
    pub cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Banishment {
    pub cards: Vec<Card>,
    zone_info: ZoneInformation,
}

pub struct Field {
    pub cards: Vec<Card>,
    zone_info: ZoneInformation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ZoneInformation {
    visibility: Visibility,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Visibility {
    PUBLIC,
    PRIVATE,
}

pub trait Zone {
    fn add_card(&mut self, card: &Card);
    fn remove_card(&mut self, card: &Card);
    fn get_cards(&self) -> &Vec<Card>;
    fn check_card(&self, card: &Card) -> bool;
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PRIVATE,
            },
        }
    }
}

impl Zone for Hand {
    fn add_card(&mut self, card: &Card) {
        self.cards.push(card.clone());
    }

    fn remove_card(&mut self, card: &Card) {
        self.cards.retain_mut(|card_in_hand| card != card_in_hand);
    }

    fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn check_card(&self, card: &Card) -> bool {
        let cards: &Vec<Card> = &self.cards;

        true
    }
}

impl Zone for Graveyard {
    fn add_card(&mut self, card: &Card) {
        self.cards.push(card.clone());
    }

    fn remove_card(&mut self, card: &Card) {
        self.cards.retain_mut(|card_in_hand| card != card_in_hand);
    }

    fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn check_card(&self, card: &Card) -> bool {
        let cards: &Vec<Card> = &self.cards;

        true
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

impl Field {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PUBLIC,
            },
        }
    }
}

impl Graveyard {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PUBLIC,
            },
        }
    }
}

impl Banishment {
    pub fn new() -> Self {
        Self {
            cards: Vec::new(),
            zone_info: ZoneInformation {
                visibility: Visibility::PUBLIC,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZoneChangeError {
    error_message: String,
}

impl fmt::Display for ZoneChangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while trying to move card between zones")
    }
}

pub fn move_zones<T: Zone, K: Zone>(
    from_zone: &mut T,
    to_zone: &mut K,
    card: &Card,
) -> Result<(), ZoneChangeError> {
    let matched_card = match from_zone
        .get_cards()
        .iter()
        .find(|&card_in_hand| *card_in_hand == *card)
    {
        Some(card) => Ok(card),
        None => Err(ZoneChangeError {
            error_message: "Tried to remove card from hand that did not exist".to_string(),
        }),
    };

    if let Ok(_) = matched_card {
        to_zone.add_card(&card);
        from_zone.remove_card(&card);
    } else {
        return Err(ZoneChangeError {
            error_message: "Error while trying to move card between zones".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::import_cards;

    use super::*;

    #[test]
    fn card_zone_move_success() {
        let mut number_is_correct = true;
        let cards = import_cards().unwrap();
        let mut hand = Hand::new();
        let mut gy = Graveyard::new();
        let banishment = Banishment::new();
        let card = Card::from(&cards[0]);

        hand.add_card(&card);

        move_zones(&mut hand, &mut gy, &card);

        println!(
            "Cards in hand: {}, Cards in gy: {:?}",
            hand.cards.len(),
            gy.cards
        );

        if hand.cards.len() != 0 {
            number_is_correct = false;
        }

        if gy.cards.len() != 1 {
            number_is_correct = false;
        }

        assert!(number_is_correct);
    }

    #[test]
    fn card_zone_move_correct_card_was_moved() {
        let cards = import_cards().unwrap();
        let mut hand = Hand::new();
        let mut gy = Graveyard::new();
        let banishment = Banishment::new();
        let card = Card::from(&cards[0]);
        let card_before = card.clone();

        hand.add_card(&card);

        move_zones(&mut hand, &mut gy, &card);

        println!(
            "Cards in hand: {}, Cards in gy: {:?}",
            hand.cards.len(),
            gy.cards
        );

        assert!(card_before == gy.cards[0]);
    }
}
