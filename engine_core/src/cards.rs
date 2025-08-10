use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Player, zones::Zone};

// TODO: Cards need to know which zone they're in somehow
// TODO: Decide on when and how to set card ownership.
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub owning_player: Option<Player>,
    pub card_info: CardInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardInfo {
    pub classes: Vec<Class>,
    pub cost_memory: Option<i32>,
    pub cost_reserve: Option<i32>,
    pub durability: Option<i32>,
    pub editions: Vec<EditionInfo>,
    pub effect: Option<String>,
    pub element: Element,
    pub elements: Vec<Element>,
    pub flavor: Option<String>,
    pub last_update: DateTime<Utc>,
    pub level: Option<i32>,
    pub life: Option<i32>,
    pub name: String,
    pub power: Option<i32>,
    pub referenced_by: Vec<CardRefernce>,
    pub references: Vec<CardRefernce>,
    pub result_editions: Vec<EditionInfo>,
    pub rule: Vec<Ruling>,
    pub slug: String,
    pub speed: Option<bool>,
    pub subtypes: Vec<Subtype>,
    pub uuid: String,
}

// TODO: Give kind and direction proper enum types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardRefernce {
    kind: String,
    name: String,
    slug: String,
    direction: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ruling {
    title: String,
    date_added: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Class {
    ASSASSIN,
    CLERIC,
    GUARDIAN,
    MAGE,
    RANGER,
    TAMER,
    SPIRIT,
    WARRIOR,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Type {
    ACTION,
    ALLY,
    ATTACK,
    CHAMPION,
    DOMAIN,
    ITEM,
    MASTERY,
    PHANTASIA,
    REGALIA,
    TOKEN,
    UNIQUE,
    WEAPON,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Subtype {
    ACCESSORY,
    ADJUVANT,
    AETHERCHARGE,
    AETHERWING,
    ANGEL,
    ANIMAL,
    ANTELOPE,
    APE,
    APPARITION,
    ARMOR,
    ARROW,
    ARTIFACT,
    ASSASSIN,
    AUTOMATON,
    AVATAR,
    AXE,
    BAUBLE,
    BEAR,
    BEAST,
    BIRD,
    BOAR,
    BOOK,
    BOOTS,
    BOW,
    BULL,
    BULLET,
    CASTLE,
    CAT,
    CATACLYSM,
    CATALYST,
    CHESSMAN,
    CLERIC,
    CLOAK,
    COMMAND,
    COMPONENT,
    CONSTRUCT,
    CRAFT,
    CROSSROADS,
    CRYSTAL,
    CURSE,
    DAGGER,
    DEER,
    DEVICE,
    DISTORTION,
    DOG,
    DOLPHIN,
    DRAGON,
    DRYAD,
    ELEMENTAL,
    FACTORY,
    FAIRY,
    FAN,
    FARM,
    FATEBOUND,
    FATESTONE,
    FISH,
    FIST,
    FLOWER,
    FLOWERBUD,
    FLUTE,
    FOOD,
    FOX,
    FRACTAL,
    FROG,
    GATE,
    GLOVES,
    GOLEM,
    GUARDIAN,
    GUN,
    HAMMER,
    HARMONY,
    HERB,
    HORN,
    HORSE,
    HUMAN,
    INSTRUMENT,
    ISLE,
    KING,
    KINGDOM,
    LASH,
    LEAF,
    LION,
    MAGE,
    MAP,
    MARKET,
    MAUL,
    MELODY,
    MONKEY,
    MOUSE,
    MUSHROOM,
    OBELISK,
    OTTER,
    PANGOLIN,
    PARTY,
    PHOENIX,
    POLEARM,
    POTION,
    POWERCELL,
    QUEEN,
    RABBIT,
    RACCOON,
    RANGER,
    REACTION,
    RHINO,
    RING,
    RIVER,
    ROBE,
    ROOK,
    ROOT,
    RUINS,
    SCEPTER,
    SCRIPTURE,
    SELKIE,
    SERPENT,
    SHADOW,
    SHARD,
    SHEEP,
    SHENJU,
    SHIELD,
    SIEGEABLE,
    SKILL,
    SLIME,
    SNAKE,
    SOLVENT,
    SPECTER,
    SPELL,
    SPIRE,
    SPIRIT,
    SQUIRREL,
    STAFF,
    SUITED,
    SWORD,
    TAMER,
    THRONE,
    TIGER,
    TURTLE,
    UNICORN,
    WAND,
    WARRIOR,
    WHALE,
    WHIP,
    WOLF,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Element {
    ARCANE,
    ASTRA,
    CRUX,
    EXALTED,
    EXIA,
    FIRE,
    LUXEM,
    NEOS,
    NORM,
    TERA,
    UMBRA,
    WATER,
    WIND,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditionInfo {}

pub trait CardBehavior {
    fn change_zone<T: Zone>(zone: T) {
        // stub
    }
}

impl From<CardInfo> for Card {
    fn from(card_info: CardInfo) -> Self {
        Self {
            owning_player: None,
            card_info: card_info,
        }
    }
}
