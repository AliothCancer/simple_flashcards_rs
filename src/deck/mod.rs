mod card;
mod deck;
mod repetition_strategy;

pub use card::*;
pub use deck::*;
pub use repetition_strategy::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Subject{
    pub(crate) name: String,
    pub(crate) arguments: Vec<Argument>
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Argument {
    pub(crate) name: String,
    pub(crate) flashcards: Vec<Card>
}


