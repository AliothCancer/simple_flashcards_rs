use crate::deck::card::*;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deck {
    cards: Vec<Card>
}
