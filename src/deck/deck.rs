use crate::deck::card::*;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deck {
    pub cards: Vec<Card>
}
