use super::card::Card;
use super::deck::Deck;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum GuessScore {
    Easy,
    Hard,
    CouldNotRemember,
}

pub trait RepetitionStrategy {
    fn prepare_deck_repetition(&mut self, deck: &Deck);
    fn next_card(&mut self) -> Option<Card>;
    fn inform_card_score(&mut self, card: &Card, guess_score: GuessScore);
}

pub struct LinearRepetitionStrategy {
    cards_to_repeat: Vec<Card>
}

impl RepetitionStrategy for LinearRepetitionStrategy {
    fn prepare_deck_repetition(&mut self, deck: &Deck) {
        self.cards_to_repeat = deck.cards.clone();
    }

    fn next_card(&mut self) -> Option<Card> {
        self.cards_to_repeat.pop()
    }

    fn inform_card_score(&mut self, card: &Card, guess_score: GuessScore) {
        if guess_score == GuessScore::CouldNotRemember {
            self.cards_to_repeat.push(card.clone()); 
        }
    }
}
