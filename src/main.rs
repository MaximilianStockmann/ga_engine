use engine_core::parser::{self, import_cards};

fn main() {
    let cards = match import_cards() {
        Ok(cards) => cards,
        Err(e) => panic!("{}", e),
    };

    println!(
        "Card is named {} and has the classes {:?}. It was last updated on {}",
        cards[0].name, cards[0].classes, cards[0].last_update
    );

    println!("Test");
}
