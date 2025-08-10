use engine_core::{parser::import_cards, zones::MainDeck};
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

    game.players[0].hand.add_card(cards[100].clone());
    game.players[1].main_deck = MainDeck::new_random(cards);
    game.players[1].draw_from_deck(7);

    let player_json = serde_json::to_string(&game.players[1].hand).unwrap();

    println!("Players in game: {}", player_json);
}
