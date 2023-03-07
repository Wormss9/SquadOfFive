mod value;
use rand::{seq::SliceRandom, thread_rng};
use value::Value;
mod card;
pub use card::Card;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Play {
    cards: Vec<Card>,
}

impl Play {
    pub fn new(mut cards: Vec<Card>) -> Self {
        cards.sort_unstable_by(|a, b| b.cmp(a));
        Self { cards }
    }

    pub fn beats(&self, other: &Self) -> bool {
        match other.value() {
            Value::None => self.value() > Value::None,
            Value::Single | Value::Pair | Value::Three => {
                (self.value() == other.value()
                    && self.compare_same_value(other) == Ordering::Greater)
                    || self.value() >= Value::FourOfAKind
            }
            Value::Straight => {
                (self.value() == Value::Straight
                    && self.compare_same_value(other) == Ordering::Greater)
                    || self.value() > Value::Straight
            }
            Value::Flush => {
                (self.value() == Value::Flush
                    && self.compare_same_value(other) == Ordering::Greater)
                    || self.value() > Value::Flush
            }
            Value::FullHouse => self.beats_full_house(other) || self.value() > Value::FullHouse,
            Value::StraightFlush => {
                (self.value() == Value::StraightFlush
                    && self.compare_same_value(other) == Ordering::Greater)
                    || (self.value() > Value::StraightFlush)
            }
            Value::FourOfAKind => {
                self.value() > other.value()
                    || (self.value() == Value::FourOfAKind
                        && self.compare_same_value(other) == Ordering::Greater)
            }
            Value::FiveOfAKind => {
                self.value() > other.value()
                    || (self.value() == Value::FiveOfAKind
                        && self.compare_same_value(other) == Ordering::Greater)
            }
            Value::SixOfAKind => {
                self.value() > other.value()
                    || (self.value() == Value::SixOfAKind
                        && self.compare_same_value(other) == Ordering::Greater)
            }
            Value::SevenOfAKind => false,
        }
    }

    fn same_value(&self) -> bool {
        let first_card = &self.cards[0];
        self.cards
            .clone()
            .iter()
            .filter(|card| card.value != first_card.value)
            .count()
            == 0
    }

    fn compare_same_value(&self, other: &Self) -> Ordering {
        if self.cards.len() != other.cards.len() {
            Ordering::Less
        } else {
            for x in 0..self.cards.len() {
                let comparison = self.cards[x].cmp(&other.cards[x]);
                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
            Ordering::Equal
        }
    }

    fn beats_full_house(&self, other: &Self) -> bool {
        let x = self.full_house();
        let y = other.full_house();

        if !x.0 || !y.0 {
            false
        } else {
            match x.1.compare_same_value(&y.1) {
                Ordering::Less => false,
                Ordering::Equal => x.2.compare_same_value(&y.2) == Ordering::Greater,
                Ordering::Greater => true,
            }
        }
    }

    fn same_color(&self) -> bool {
        let card_color = match self.cards.clone().iter().find(|x| x.color.to_char() != 'w') {
            Some(card) => card.color.to_char(),
            None => 'w',
        };
        self.cards
            .clone()
            .iter()
            .filter(|card| !(card.color.to_char() == card_color || card.color.to_char() == 'w'))
            .count()
            == 0
    }

    fn straight(&self) -> bool {
        for x in 0..(self.cards.len() - 1) {
            if (self.cards[x].value.to_u8() as i8 - self.cards[x + 1].value.to_u8() as i8).abs()
                != 1
                || self.cards[x].value.to_u8() >= 11
            {
                return false;
            }
        }
        true
    }

    fn full_house(&self) -> (bool, Self, Self) {
        if self.cards.len() != 5 {
            (false, Play { cards: vec![] }, Play { cards: vec![] })
        } else if Play::new(self.cards[0..3].to_vec()).value() == Value::Three
            && Play::new(self.cards[3..5].to_vec()).value() == Value::Pair
        {
            (
                true,
                Play::new(self.cards[0..3].to_vec()),
                Play::new(self.cards[3..5].to_vec()),
            )
        } else if Play::new(self.cards[0..2].to_vec()).value() == Value::Pair
            && Play::new(self.cards[2..5].to_vec()).value() == Value::Three
        {
            (
                true,
                Play::new(self.cards[2..5].to_vec()),
                Play::new(self.cards[0..2].to_vec()),
            )
        } else {
            (false, Play { cards: vec![] }, Play { cards: vec![] })
        }
    }

    fn value(&self) -> Value {
        match self.cards.len() {
            1 => Value::Single,
            2 => {
                if self.same_value() {
                    Value::Pair
                } else {
                    Value::None
                }
            }
            3 => {
                if self.same_value() {
                    Value::Three
                } else {
                    Value::None
                }
            }
            4 => {
                if self.same_value() {
                    Value::FourOfAKind
                } else {
                    Value::None
                }
            }
            5 => {
                if self.same_value() {
                    Value::FiveOfAKind
                } else if self.same_color() && self.straight() {
                    Value::StraightFlush
                } else if self.full_house().0 {
                    Value::FullHouse
                } else if self.same_color() {
                    Value::Flush
                } else if self.straight() {
                    Value::Straight
                } else {
                    Value::None
                }
            }
            6 => {
                if self.same_value() {
                    Value::SixOfAKind
                } else {
                    Value::None
                }
            }
            7 => {
                if self.same_value() {
                    Value::SevenOfAKind
                } else {
                    Value::None
                }
            }
            _ => Value::None,
        }
    }
}

pub fn deal_cards() -> [Vec<Card>; 4] {
    let mut deck: Vec<Card> = vec![];
    for _ in 0..2 {
        for color in 1..4 {
            for value in 1..11 {
                deck.push(Card::new(value, color));
            }
        }
    }

    deck.push(Card::new(11, 1));
    deck.push(Card::new(11, 2));
    deck.push(Card::new(12, 3));
    deck.push(Card::new(1, 4));

    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    let mut hands = [
        deck[0..16].to_vec(),
        deck[16..32].to_vec(),
        deck[32..48].to_vec(),
        deck[48..64].to_vec(),
    ];

    for cards in hands.as_mut() {
        cards.sort();
    }

    hands
}

#[cfg(test)]
mod tests {
    use std::vec;

    struct Plays {
        empty: Play,
        single1: Play,
        single2: Play,
        pair1: Play,
        pair2: Play,
        pair3: Play,
        notpair: Play,
        toak1: Play,
        toak2: Play,
        toak3: Play,
        nottoak: Play,
        straight1: Play,
        straight2: Play,
        straight3: Play,
        flush1: Play,
        flush2: Play,
        flush3: Play,
        fullhouse1: Play,
        fullhouse2: Play,
        fullhouse3: Play,
        fullhouse4: Play,
        fullhouse5: Play,
        sf1: Play,
        sf2: Play,
        sf3: Play,
        nfkk1: Play,
        nfkk2: Play,
        nfkk3: Play,
        foak1: Play,
        foak2: Play,
        fioak1: Play,
        fioak2: Play,
        soak1: Play,
        soak2: Play,
        seoak: Play,
    }

    impl Plays {
        fn new() -> Self {
            Self {
                empty: Play::new(vec![]),
                single1: Play::new(vec![Card::new(1, 3)]),
                single2: Play::new(vec![Card::new(1, 4)]),
                pair1: Play::new(vec![Card::new(1, 2), Card::new(1, 3)]),
                pair2: Play::new(vec![Card::new(1, 3), Card::new(1, 4)]),
                pair3: Play::new(vec![Card::new(2, 1), Card::new(2, 2)]),
                notpair: Play::new(vec![Card::new(2, 1), Card::new(3, 2)]),
                toak1: Play::new(vec![Card::new(1, 1), Card::new(1, 2), Card::new(1, 3)]),
                toak2: Play::new(vec![Card::new(1, 2), Card::new(1, 3), Card::new(1, 4)]),
                toak3: Play::new(vec![Card::new(2, 1), Card::new(2, 2), Card::new(2, 3)]),
                nottoak: Play::new(vec![Card::new(1, 1), Card::new(2, 1), Card::new(3, 1)]),
                straight1: Play::new(vec![
                    Card::new(1, 2),
                    Card::new(2, 2),
                    Card::new(3, 1),
                    Card::new(4, 3),
                    Card::new(5, 2),
                ]),
                straight2: Play::new(vec![
                    Card::new(1, 4),
                    Card::new(2, 2),
                    Card::new(3, 1),
                    Card::new(4, 3),
                    Card::new(5, 2),
                ]),
                straight3: Play::new(vec![
                    Card::new(2, 1),
                    Card::new(3, 2),
                    Card::new(4, 1),
                    Card::new(5, 1),
                    Card::new(6, 1),
                ]),
                flush1: Play::new(vec![
                    Card::new(1, 1),
                    Card::new(1, 1),
                    Card::new(3, 1),
                    Card::new(4, 1),
                    Card::new(5, 1),
                ]),
                flush2: Play::new(vec![
                    Card::new(1, 4),
                    Card::new(1, 1),
                    Card::new(3, 1),
                    Card::new(4, 1),
                    Card::new(5, 1),
                ]),
                flush3: Play::new(vec![
                    Card::new(1, 4),
                    Card::new(1, 1),
                    Card::new(3, 1),
                    Card::new(4, 1),
                    Card::new(6, 1),
                ]),
                fullhouse1: Play::new(vec![
                    Card::new(10, 1),
                    Card::new(10, 1),
                    Card::new(3, 1),
                    Card::new(3, 1),
                    Card::new(3, 1),
                ]),
                fullhouse2: Play::new(vec![
                    Card::new(1, 3),
                    Card::new(1, 3),
                    Card::new(4, 1),
                    Card::new(4, 1),
                    Card::new(4, 2),
                ]),
                fullhouse3: Play::new(vec![
                    Card::new(1, 1),
                    Card::new(1, 1),
                    Card::new(4, 1),
                    Card::new(4, 2),
                    Card::new(4, 2),
                ]),
                fullhouse4: Play::new(vec![
                    Card::new(5, 1),
                    Card::new(5, 2),
                    Card::new(5, 1),
                    Card::new(2, 3),
                    Card::new(2, 2),
                ]),
                fullhouse5: Play::new(vec![
                    Card::new(11, 1),
                    Card::new(11, 2),
                    Card::new(10, 3),
                    Card::new(10, 3),
                    Card::new(10, 2),
                ]),
                sf1: Play::new(vec![
                    Card::new(5, 1),
                    Card::new(4, 1),
                    Card::new(3, 1),
                    Card::new(2, 1),
                    Card::new(1, 1),
                ]),
                sf2: Play::new(vec![
                    Card::new(5, 2),
                    Card::new(4, 2),
                    Card::new(3, 2),
                    Card::new(2, 2),
                    Card::new(1, 2),
                ]),
                sf3: Play::new(vec![
                    Card::new(6, 1),
                    Card::new(5, 1),
                    Card::new(4, 1),
                    Card::new(3, 1),
                    Card::new(2, 1),
                ]),
                nfkk1: Play::new(vec![
                    Card::new(11, 1),
                    Card::new(10, 2),
                    Card::new(10, 3),
                    Card::new(10, 3),
                    Card::new(10, 2),
                ]),
                nfkk2: Play::new(vec![
                    Card::new(12, 3),
                    Card::new(11, 2),
                    Card::new(10, 3),
                    Card::new(9, 3),
                    Card::new(8, 3),
                ]),
                nfkk3: Play::new(vec![
                    Card::new(11, 2),
                    Card::new(10, 3),
                    Card::new(9, 3),
                    Card::new(8, 3),
                    Card::new(7, 3),
                ]),
                foak1: Play::new(vec![
                    Card::new(8, 1),
                    Card::new(8, 2),
                    Card::new(8, 3),
                    Card::new(8, 3),
                ]),
                foak2: Play::new(vec![
                    Card::new(9, 1),
                    Card::new(9, 2),
                    Card::new(9, 2),
                    Card::new(9, 3),
                ]),
                fioak1: Play::new(vec![
                    Card::new(7, 1),
                    Card::new(7, 2),
                    Card::new(7, 2),
                    Card::new(7, 3),
                    Card::new(7, 3),
                ]),
                fioak2: Play::new(vec![
                    Card::new(8, 1),
                    Card::new(8, 1),
                    Card::new(8, 2),
                    Card::new(8, 2),
                    Card::new(8, 3),
                ]),
                soak1: Play::new(vec![
                    Card::new(8, 1),
                    Card::new(8, 1),
                    Card::new(8, 2),
                    Card::new(8, 2),
                    Card::new(8, 3),
                    Card::new(8, 3),
                ]),
                soak2: Play::new(vec![
                    Card::new(10, 1),
                    Card::new(10, 1),
                    Card::new(10, 2),
                    Card::new(10, 2),
                    Card::new(10, 3),
                    Card::new(10, 3),
                ]),
                seoak: Play::new(vec![
                    Card::new(1, 1),
                    Card::new(1, 1),
                    Card::new(1, 2),
                    Card::new(1, 2),
                    Card::new(1, 3),
                    Card::new(1, 3),
                    Card::new(1, 4),
                ]),
            }
        }
    }

    use super::*;
    #[test]
    fn play_value() {
        let plays = Plays::new();
        assert_eq!(plays.empty.value(), Value::None);
        assert_eq!(plays.single1.value(), Value::Single);
        assert_eq!(plays.single2.value(), Value::Single);
        assert_eq!(plays.pair1.value(), Value::Pair);
        assert_eq!(plays.pair2.value(), Value::Pair);
        assert_eq!(plays.pair3.value(), Value::Pair);
        assert_eq!(plays.notpair.value(), Value::None);
        assert_eq!(plays.toak1.value(), Value::Three);
        assert_eq!(plays.toak2.value(), Value::Three);
        assert_eq!(plays.toak3.value(), Value::Three);
        assert_eq!(plays.nottoak.value(), Value::None);
        assert_eq!(plays.nottoak.value(), Value::None);
        assert_eq!(plays.straight1.value(), Value::Straight);
        assert_eq!(plays.straight2.value(), Value::Straight);
        assert_eq!(plays.straight3.value(), Value::Straight);
        assert_eq!(plays.flush1.value(), Value::Flush);
        assert_eq!(plays.flush2.value(), Value::Flush);
        assert_eq!(plays.flush3.value(), Value::Flush);
        assert_eq!(plays.fullhouse1.value(), Value::FullHouse);
        assert_eq!(plays.fullhouse2.value(), Value::FullHouse);
        assert_eq!(plays.fullhouse3.value(), Value::FullHouse);
        assert_eq!(plays.fullhouse4.value(), Value::FullHouse);
        assert_eq!(plays.fullhouse5.value(), Value::FullHouse);
        assert_eq!(plays.sf1.value(), Value::StraightFlush);
        assert_eq!(plays.sf2.value(), Value::StraightFlush);
        assert_eq!(plays.sf3.value(), Value::StraightFlush);
        assert_eq!(plays.foak1.value(), Value::FourOfAKind);
        assert_eq!(plays.foak2.value(), Value::FourOfAKind);
        assert_eq!(plays.fioak1.value(), Value::FiveOfAKind);
        assert_eq!(plays.fioak2.value(), Value::FiveOfAKind);
        assert_eq!(plays.nfkk1.value(), Value::None);
        assert_eq!(plays.nfkk2.value(), Value::None);
        assert_eq!(plays.nfkk3.value(), Value::None);
        assert_eq!(plays.soak1.value(), Value::SixOfAKind);
        assert_eq!(plays.soak2.value(), Value::SixOfAKind);
        assert_eq!(plays.seoak.value(), Value::SevenOfAKind);
    }
    #[test]
    fn play_comparison() {
        let plays = Plays::new();
        assert!(plays.seoak.beats(&plays.soak1));
        assert!(plays.seoak.beats(&plays.fioak1));
        assert!(plays.seoak.beats(&plays.foak1));
        assert!(plays.seoak.beats(&plays.sf2));
        assert!(plays.seoak.beats(&plays.fullhouse1));
        assert!(plays.seoak.beats(&plays.flush1));
        assert!(plays.seoak.beats(&plays.straight1));
        assert!(plays.seoak.beats(&plays.toak1));
        assert!(plays.seoak.beats(&plays.pair1));
        assert!(plays.seoak.beats(&plays.single1));
        assert!(plays.seoak.beats(&plays.empty));

        assert!(plays.soak2.beats(&plays.soak1));
        assert!(plays.soak1.beats(&plays.fioak1));
        assert!(plays.soak1.beats(&plays.foak1));
        assert!(plays.soak1.beats(&plays.sf2));
        assert!(plays.soak1.beats(&plays.fullhouse1));
        assert!(plays.soak1.beats(&plays.flush1));
        assert!(plays.soak1.beats(&plays.straight1));
        assert!(plays.soak1.beats(&plays.toak1));
        assert!(plays.soak1.beats(&plays.pair1));
        assert!(plays.soak1.beats(&plays.single1));
        assert!(plays.soak1.beats(&plays.empty));

        assert!(plays.fioak2.beats(&plays.fioak1));
        assert!(plays.fioak1.beats(&plays.foak1));
        assert!(plays.fioak1.beats(&plays.sf2));
        assert!(plays.fioak1.beats(&plays.fullhouse1));
        assert!(plays.fioak1.beats(&plays.flush1));
        assert!(plays.fioak1.beats(&plays.straight1));
        assert!(plays.fioak1.beats(&plays.toak1));
        assert!(plays.fioak1.beats(&plays.pair1));
        assert!(plays.fioak1.beats(&plays.single1));
        assert!(plays.fioak1.beats(&plays.empty));

        assert!(plays.foak2.beats(&plays.foak1));
        assert!(plays.foak1.beats(&plays.sf2));
        assert!(plays.foak1.beats(&plays.fullhouse1));
        assert!(plays.foak1.beats(&plays.flush1));
        assert!(plays.foak1.beats(&plays.straight1));
        assert!(plays.foak1.beats(&plays.toak1));
        assert!(plays.foak1.beats(&plays.pair1));
        assert!(plays.foak1.beats(&plays.single1));
        assert!(plays.foak1.beats(&plays.empty));

        assert!(plays.sf3.beats(&plays.sf2));
        assert!(plays.sf2.beats(&plays.sf1));
        assert!(plays.sf1.beats(&plays.fullhouse1));
        assert!(plays.sf1.beats(&plays.flush1));
        assert!(plays.sf1.beats(&plays.straight1));
        assert!(plays.sf1.beats(&plays.empty));

        assert!(plays.fullhouse5.beats(&plays.fullhouse4));
        assert!(plays.fullhouse4.beats(&plays.fullhouse3));
        assert!(plays.fullhouse3.beats(&plays.fullhouse2));
        assert!(plays.fullhouse2.beats(&plays.fullhouse1));
        assert!(plays.sf1.beats(&plays.flush1));
        assert!(plays.sf1.beats(&plays.straight1));
        assert!(plays.sf1.beats(&plays.empty));

        assert!(plays.sf3.beats(&plays.sf2));
        assert!(plays.sf2.beats(&plays.sf1));
        assert!(plays.sf1.beats(&plays.flush1));
        assert!(plays.sf1.beats(&plays.straight1));
        assert!(plays.sf1.beats(&plays.empty));

        assert!(plays.flush3.beats(&plays.flush2));
        assert!(plays.flush2.beats(&plays.flush1));
        assert!(plays.flush1.beats(&plays.straight1));
        assert!(plays.flush1.beats(&plays.empty));

        assert!(plays.straight3.beats(&plays.straight2));
        assert!(plays.straight2.beats(&plays.straight1));
        assert!(plays.straight1.beats(&plays.empty));

        assert!(plays.toak3.beats(&plays.toak2));
        assert!(plays.toak2.beats(&plays.toak1));
        assert!(plays.toak1.beats(&plays.empty));

        assert!(plays.pair3.beats(&plays.pair2));
        assert!(plays.pair2.beats(&plays.pair1));
        assert!(plays.pair1.beats(&plays.empty));

        assert!(plays.single2.beats(&plays.single1));
        assert!(plays.pair1.beats(&plays.empty));

        ////////////////////////////

        assert!(!plays.soak1.beats(&plays.seoak));
        assert!(!plays.fioak1.beats(&plays.seoak));
        assert!(!plays.foak1.beats(&plays.seoak));
        assert!(!plays.sf2.beats(&plays.seoak));
        assert!(!plays.fullhouse1.beats(&plays.seoak));
        assert!(!plays.flush1.beats(&plays.seoak));
        assert!(!plays.straight1.beats(&plays.seoak));
        assert!(!plays.toak1.beats(&plays.seoak));
        assert!(!plays.pair1.beats(&plays.seoak));
        assert!(!plays.single1.beats(&plays.seoak));
        assert!(!plays.empty.beats(&plays.seoak));

        assert!(!plays.soak1.beats(&plays.soak2));
        assert!(!plays.fioak1.beats(&plays.soak1));
        assert!(!plays.foak1.beats(&plays.soak1));
        assert!(!plays.sf2.beats(&plays.soak1));
        assert!(!plays.fullhouse1.beats(&plays.soak1));
        assert!(!plays.flush1.beats(&plays.soak1));
        assert!(!plays.straight1.beats(&plays.soak1));
        assert!(!plays.toak1.beats(&plays.soak1));
        assert!(!plays.pair1.beats(&plays.soak1));
        assert!(!plays.single1.beats(&plays.soak1));
        assert!(!plays.empty.beats(&plays.soak1));

        assert!(!plays.fioak1.beats(&plays.fioak2));
        assert!(!plays.foak1.beats(&plays.fioak1));
        assert!(!plays.sf2.beats(&plays.fioak1));
        assert!(!plays.fullhouse1.beats(&plays.fioak1));
        assert!(!plays.flush1.beats(&plays.fioak1));
        assert!(!plays.straight1.beats(&plays.fioak1));
        assert!(!plays.toak1.beats(&plays.fioak1));
        assert!(!plays.pair1.beats(&plays.fioak1));
        assert!(!plays.single1.beats(&plays.fioak1));
        assert!(!plays.empty.beats(&plays.fioak1));

        assert!(!plays.foak1.beats(&plays.foak2));
        assert!(!plays.sf2.beats(&plays.foak1));
        assert!(!plays.fullhouse1.beats(&plays.foak1));
        assert!(!plays.flush1.beats(&plays.foak1));
        assert!(!plays.straight1.beats(&plays.foak1));
        assert!(!plays.toak1.beats(&plays.foak1));
        assert!(!plays.pair1.beats(&plays.foak1));
        assert!(!plays.single1.beats(&plays.foak1));
        assert!(!plays.empty.beats(&plays.foak1));

        assert!(!plays.sf2.beats(&plays.sf3));
        assert!(!plays.sf1.beats(&plays.sf2));
        assert!(!plays.fullhouse1.beats(&plays.sf1));
        assert!(!plays.flush1.beats(&plays.sf1));
        assert!(!plays.straight1.beats(&plays.sf1));
        assert!(!plays.toak1.beats(&plays.sf1));
        assert!(!plays.pair1.beats(&plays.sf1));
        assert!(!plays.single1.beats(&plays.sf1));
        assert!(!plays.empty.beats(&plays.sf1));

        assert!(!plays.fullhouse4.beats(&plays.fullhouse5));
        assert!(!plays.fullhouse3.beats(&plays.fullhouse4));
        assert!(!plays.fullhouse2.beats(&plays.fullhouse3));
        assert!(!plays.fullhouse1.beats(&plays.fullhouse2));
        assert!(!plays.flush1.beats(&plays.sf1));
        assert!(!plays.straight1.beats(&plays.sf1));
        assert!(!plays.toak1.beats(&plays.sf1));
        assert!(!plays.pair1.beats(&plays.sf1));
        assert!(!plays.single1.beats(&plays.sf1));
        assert!(!plays.empty.beats(&plays.sf1));

        assert!(!plays.sf2.beats(&plays.sf3));
        assert!(!plays.sf1.beats(&plays.sf2));
        assert!(!plays.flush1.beats(&plays.sf1));
        assert!(!plays.straight1.beats(&plays.sf1));
        assert!(!plays.toak1.beats(&plays.sf1));
        assert!(!plays.pair1.beats(&plays.sf1));
        assert!(!plays.single1.beats(&plays.sf1));
        assert!(!plays.empty.beats(&plays.sf1));

        assert!(!plays.flush2.beats(&plays.flush3));
        assert!(!plays.flush1.beats(&plays.flush2));
        assert!(!plays.straight1.beats(&plays.flush1));
        assert!(!plays.toak1.beats(&plays.flush1));
        assert!(!plays.pair1.beats(&plays.flush1));
        assert!(!plays.single1.beats(&plays.flush1));
        assert!(!plays.empty.beats(&plays.flush1));

        assert!(!plays.straight2.beats(&plays.straight3));
        assert!(!plays.straight1.beats(&plays.straight2));
        assert!(!plays.toak1.beats(&plays.straight1));
        assert!(!plays.pair1.beats(&plays.straight1));
        assert!(!plays.single1.beats(&plays.straight1));
        assert!(!plays.empty.beats(&plays.straight1));

        assert!(!plays.toak2.beats(&plays.toak3));
        assert!(!plays.toak1.beats(&plays.toak2));
        assert!(!plays.pair1.beats(&plays.toak1));
        assert!(!plays.single1.beats(&plays.toak1));
        assert!(!plays.empty.beats(&plays.toak1));

        assert!(!plays.pair2.beats(&plays.pair3));
        assert!(!plays.pair1.beats(&plays.pair2));
        assert!(!plays.single1.beats(&plays.pair1));
        assert!(!plays.empty.beats(&plays.pair1));

        assert!(!plays.single1.beats(&plays.single2));
        assert!(!plays.empty.beats(&plays.pair1));
    }
}
