use std::{cell::RefCell, rc::Rc};

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
    pub hand: Hand,
    pub main_deck: MainDeck,
    pub material_deck: MaterialDeck,
    pub graveyard: Graveyard,
    pub banishment: Banishment,
}

pub struct Game {
    pub players: Vec<RefCell<Player>>,
    pub field: Field,
    pub turn_player: RefCell<Player>,
}

impl Game {
    fn new(player_names: Vec<&str>) -> Self {
        let players = player_names
            .iter()
            .map(|name| Player {
                name: name.to_string(),
                hand: Hand::new(),
                main_deck: MainDeck::new(),
                material_deck: MaterialDeck::new(),
                graveyard: Graveyard::new(),
                banishment: Banishment::new(),
            })
            .map(|player| RefCell::new(player))
            .collect::<Vec<RefCell<Player>>>();

        let turn_player = RefCell::clone(&players[0]);

        Self {
            players: players,
            field: Field::new(),
            turn_player: turn_player,
        }
    }

    fn proceed_turn() {
        //stub
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

    pub fn discard(&mut self, card_number: usize) -> Result<(), ZoneChangeError> {
        let card = self.hand.cards[card_number].clone();

        move_zones(&mut self.hand, &mut self.graveyard, &card)
    }
}

#[cfg(test)]
mod test {
    use crate::{init_game, parser::import_cards, zones::MainDeck};

    #[test]
    fn create_game() {
        let game = init_game();
    }

    #[test]
    fn player_discard() {
        let cards = match import_cards() {
            Ok(cards) => cards,
            Err(e) => panic!("{}", e),
        };

        let mut game = init_game();

        game.players[0].borrow_mut().main_deck = MainDeck::new_random(cards);
        game.players[0].borrow_mut().draw_from_deck(7);

        let card_to_discard = game.players[0].borrow_mut().hand.cards[0].clone();

        game.players[0].borrow_mut().discard(0).unwrap();

        assert!(
            !game.players[0]
                .borrow_mut()
                .hand
                .cards
                .iter()
                .any(|hand_card| hand_card.card_info.name == card_to_discard.card_info.name)
        )
    }

    #[test]
    #[should_panic]
    fn discard_out_of_hand_index() {
        let cards = match import_cards() {
            Ok(cards) => cards,
            Err(e) => panic!("{}", e),
        };

        let mut game = init_game();

        game.players[0].borrow_mut().main_deck = MainDeck::new_random(cards);
        game.players[0].borrow_mut().draw_from_deck(7);

        let card_to_discard = game.players[0].borrow_mut().hand.cards[0].clone();

        game.players[0].borrow_mut().discard(10).unwrap();
    }
}
