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

#[derive(Debug, Default)]
pub struct LinearRepetitionStrategy {
    cards_to_repeat: Vec<Card>,
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
            // Insert on last position: repeat after all the other cards
            self.cards_to_repeat
                .insert(self.cards_to_repeat.len() - 1, card.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::{Card, Deck};

    use super::{LinearRepetitionStrategy, RepetitionStrategy};

    #[test]
    fn try_all_good() {
        let cards = vec![
            Card {
                keyword: "Ciao".to_owned(),
                description: "Hello".to_owned(),
                priority_index: 0,
                side: crate::deck::CardSide::Back,
            },
            Card {
                keyword: "Buon giorno".to_owned(),
                description: "Good morning".to_owned(),
                priority_index: 0,
                side: crate::deck::CardSide::Back,
            },
            Card {
                keyword: "Notte".to_owned(),
                description: "Night".to_owned(),
                priority_index: 0,
                side: crate::deck::CardSide::Back,
            },
        ];
        let deck = Deck { cards };

        let mut rep_strategy = LinearRepetitionStrategy::default();
        rep_strategy.prepare_deck_repetition(&deck);

        let _ = rep_strategy.next_card().unwrap();
        let _ = rep_strategy.next_card().unwrap();
        let _ = rep_strategy.next_card().unwrap();

        assert_eq!(rep_strategy.next_card(), None);
    }
    #[test]
    fn try_wrong_one() {
        let cards = vec![
            Card {
                keyword: "Ciao".to_owned(),
                description: "Hello".to_owned(),
                priority_index: 0,
                side: crate::deck::CardSide::Back,
            },
            Card {
                keyword: "Buon giorno".to_owned(),
                description: "Good morning".to_owned(),
                priority_index: 0,
                side: crate::deck::CardSide::Back,
            },
            Card {
                keyword: "Notte".to_owned(),
                description: "Night".to_owned(),
                priority_index: 0,
                side: crate::deck::CardSide::Back,
            },
        ];
        let deck = Deck { cards };

        let mut rep_strategy = LinearRepetitionStrategy::default();
        rep_strategy.prepare_deck_repetition(&deck);

        let _ = rep_strategy.next_card().unwrap();
        let wronged = rep_strategy.next_card().unwrap();

        // You're supposed to get the score from some kind of ui
        rep_strategy.inform_card_score(&wronged, crate::deck::GuessScore::CouldNotRemember); // this card is going to be repeated

        let _ = rep_strategy.next_card().unwrap();

        assert_eq!(rep_strategy.next_card().unwrap().keyword, wronged.keyword);
    }
}
