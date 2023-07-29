
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    pub(crate) keyword: String,
    pub(crate) description: String,
    pub(crate) priority_index: i32,
    pub(crate) side: CardSide
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CardSide{
    Front,
    Back
}
