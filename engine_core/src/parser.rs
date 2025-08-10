use crate::Card;
use core::fmt;
use serde_json;
use std::fs::File;
use std::io::BufReader;

pub fn import_cards() -> Result<Vec<Card>, ImportError> {
    let file = File::open("C:\\Users\\User\\Documents\\Programming\\Rust\\ga_engine\\cards.json")?;
    let reader = BufReader::new(file);

    let cards = serde_json::from_reader(reader)?;

    Result::Ok(cards)
}

// This is a bad error type right now
// TODO: Implement proper conversion from the other two errors in here
pub struct ImportError {
    msg: String,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "card import was not possible")
    }
}

impl From<serde_json::Error> for ImportError {
    fn from(initial_error: serde_json::Error) -> Self {
        println!("{}", initial_error);
        ImportError {
            msg: String::from("Problem with JSON conversion on card import"),
        }
    }
}

impl From<std::io::Error> for ImportError {
    fn from(initial_error: std::io::Error) -> Self {
        println!("{}", initial_error);
        ImportError {
            msg: String::from("Error on reading file"),
        }
    }
}
