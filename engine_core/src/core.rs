use serde::{Deserialize, Serialize};

use crate::{
    cards::Card,
    zones::{
        Banishment, Field, Graveyard, Hand, MainDeck, MaterialDeck, Zone, ZoneChangeError,
        move_zones,
    },
};

pub mod cards;
pub mod parser;
pub mod zones;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Player {
    pub name: String,
    pub is_turn_player: bool,
    pub hand: Hand,
    pub main_deck: MainDeck,
    pub material_deck: MaterialDeck,
    pub graveyard: Graveyard,
    pub banishment: Banishment,
}

pub struct Game {
    pub players: Vec<Player>,
    pub field: Field,
}

impl Game {
    fn new(player_names: Vec<&str>) -> Self {
        let players = player_names
            .iter()
            .map(|name| Player {
                name: name.to_string(),
                is_turn_player: false,
                hand: Hand::new(),
                main_deck: MainDeck::new(),
                material_deck: MaterialDeck::new(),
                graveyard: Graveyard::new(),
                banishment: Banishment::new(),
            })
            .collect::<Vec<Player>>();

        Self {
            players: players,
            field: Field::new(),
        }
    }
}

pub fn init_game() -> Game {
    let players = vec!["Player 1", "Player 2"];
    let game = Game::new(players);

    game
}

impl Player {
    pub fn draw_from_deck(&mut self, number_to_draw: i32) {
        (0..number_to_draw)
            .into_iter()
            .for_each(|_| self.hand.cards.push(self.main_deck.draw_from_top()));
    }

    pub fn move_from_hand_to_gy(&mut self, card_number: usize) -> Result<(), ZoneChangeError> {
        let card = self.hand.cards[card_number].clone();

        move_zones(&mut self.hand, &mut self.graveyard, &card)
    }
}
