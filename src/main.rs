use engine_core::{
    parser::import_cards,
    zones::{Graveyard, MainDeck, Zone, move_zones},
};
use serde_json;

// We definitely want on thread for game logic and one for UI
fn main() {
    let cards = match import_cards() {
        Ok(cards) => cards,
        Err(e) => panic!("{}", e),
    };

    println!(
        "Card is named {} and has the classes {:?}. It was last updated on {}",
        cards[0].name, cards[0].classes, cards[0].last_update
    );

    let mut game = engine_core::init_game();

    game.players[1].main_deck = MainDeck::new_random(cards);
    game.players[1].draw_from_deck(7);

    match game.players[1].move_from_hand_to_gy(3) {
        Ok(_) => (),
        Err(e) => print!("{}", e),
    }

    println!(
        "Cards in hand: {}; Cards in graveyard: {}",
        game.players[1].hand.cards.len(),
        game.players[1].graveyard.cards.len()
    );
}
