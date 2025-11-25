use engine_core::{
    parser::import_cards,
    zones::{Graveyard, MainDeck, Zone, move_zones},
};
use serde_json;

// We definitely want one thread for game logic and one for UI
fn main() {
    let cards = match import_cards() {
        Ok(cards) => cards,
        Err(e) => panic!("{}", e),
    };

    println!(
        "Card is named {} and has the classes {:?}. It was last updated on {}",
        cards[0].name, cards[0].classes, cards[0].last_update
    );

    let game = engine_core::init_game();

    game.players[1].borrow_mut().main_deck = MainDeck::new_random(cards);
    game.players[1].borrow_mut().draw_from_deck(7);

    match game.players[1].borrow_mut().discard(3) {
        Ok(_) => (),
        Err(e) => print!("{}", e),
    }

    // I noticed this is kind of a big problem
    // The usage of RefCells turns the errors that this would throw from compile time errors to runtime errors
    // I'll read up on this later
    println!(
        "Cards in hand: {}; Cards in graveyard: {}",
        game.players[1].borrow().hand.cards.len(),
        game.players[1].borrow().graveyard.cards.len()
    );
}
