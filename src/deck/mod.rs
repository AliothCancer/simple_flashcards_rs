mod card;
mod deck;

pub use card::*;
pub use deck::*;
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


