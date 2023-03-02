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
            Value::None => true,
            Value::Single | Value::Pair | Value::Three => {
                self.compare_same_value(other) == Ordering::Greater
            }
            Value::Straight => {
                self.compare_same_value(other) == Ordering::Greater
                    && other.value() == Value::Straight
            }
            Value::Flush => {
                (self.compare_same_value(other) == Ordering::Greater
                    && other.value() == Value::Flush)
                    || other.value() == Value::Straight
            }
            Value::FullHouse => {
                self.beats_full_house(other)
                    || other.value() == Value::Straight
                    || other.value() == Value::Flush
            }
            Value::StraightFlush => {
                self.compare_same_value(other) == Ordering::Greater
                    || (other.value().to_u8() <= 6 && other.value().to_u8() >= 4)
            }
            Value::FourOfAKind => {
                self.value().to_u8() > other.value().to_u8()
                    || (other.value() == Value::FourOfAKind
                        && self.compare_same_value(other) == Ordering::Greater)
            }
            Value::FiveOfAKind => {
                self.value().to_u8() > other.value().to_u8()
                    || (other.value() == Value::FiveOfAKind
                        && self.compare_same_value(other) == Ordering::Greater)
            }
            Value::SixOfAKind => {
                self.value().to_u8() > other.value().to_u8()
                    || (other.value() == Value::SixOfAKind
                        && self.compare_same_value(other) == Ordering::Greater)
            }
            Value::SevenOfAKind => true,
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
                Play::new(self.cards[0..2].to_vec()),
                Play::new(self.cards[2..5].to_vec()),
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
    for color in 1..4 {
        for value in 1..11 {
            deck.push(Card::new(color, value));
            deck.push(Card::new(color, value));
        }
        deck.push(Card::new(color, 11));
    }
    deck.push(Card::new(4, 1));
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
    use super::*;

    #[test]
    fn play_value() {
        assert_eq!(
            Play::new(vec![
                Card::new(3, 1),
                Card::new(2, 2),
                Card::new(1, 4),
                Card::new(3, 4),
                Card::new(3, 5)
            ])
            .value()
            .to_u8(),
            0
        );
        assert_eq!(Play::new(vec![Card::new(3, 1)]).value().to_u8(), 1);
        assert_eq!(
            Play::new(vec![Card::new(3, 5), Card::new(2, 5)])
                .value()
                .to_u8(),
            2
        );
        assert_eq!(
            Play::new(vec![Card::new(3, 5), Card::new(2, 5), Card::new(1, 5)])
                .value()
                .to_u8(),
            3
        );
        assert_eq!(
            Play::new(vec![
                Card::new(4, 1),
                Card::new(2, 2),
                Card::new(3, 3),
                Card::new(1, 4),
                Card::new(2, 5)
            ])
            .value()
            .to_u8(),
            4
        );
        assert_eq!(
            Play::new(vec![
                Card::new(4, 1),
                Card::new(1, 1),
                Card::new(1, 2),
                Card::new(1, 5),
                Card::new(1, 8)
            ])
            .value()
            .to_u8(),
            5
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(1, 1),
                Card::new(2, 1),
                Card::new(2, 2),
                Card::new(3, 2)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(1, 1),
                Card::new(2, 2),
                Card::new(2, 2),
                Card::new(3, 2)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(3, 1),
                Card::new(3, 2),
                Card::new(3, 3),
                Card::new(3, 4),
                Card::new(3, 5)
            ])
            .value()
            .to_u8(),
            7
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(1, 1),
                Card::new(2, 1),
                Card::new(2, 1)
            ])
            .value()
            .to_u8(),
            8
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(1, 1),
                Card::new(2, 1),
                Card::new(2, 1),
                Card::new(3, 1)
            ])
            .value()
            .to_u8(),
            9
        );
    }
    #[test]
    fn full_house() {
        assert_eq!(
            Play::new(vec![
                Card::new(1, 4),
                Card::new(4, 4),
                Card::new(3, 7),
                Card::new(3, 7),
                Card::new(4, 7)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(2, 3),
                Card::new(3, 3),
                Card::new(4, 6),
                Card::new(3, 6),
                Card::new(2, 6)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 4),
                Card::new(4, 4),
                Card::new(3, 7),
                Card::new(3, 7),
                Card::new(4, 7)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(2, 3),
                Card::new(3, 3),
                Card::new(4, 6),
                Card::new(3, 6),
                Card::new(2, 6)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 5),
                Card::new(4, 5),
                Card::new(2, 5),
                Card::new(1, 2),
                Card::new(4, 2)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(4, 11),
                Card::new(4, 11),
                Card::new(4, 9),
                Card::new(3, 9),
                Card::new(3, 9)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 5),
                Card::new(4, 5),
                Card::new(2, 5),
                Card::new(1, 2),
                Card::new(4, 2)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(4, 11),
                Card::new(4, 11),
                Card::new(4, 9),
                Card::new(3, 9),
                Card::new(3, 9)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(3, 10),
                Card::new(3, 1),
                Card::new(4, 1)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(4, 10),
                Card::new(3, 1),
                Card::new(2, 1)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(3, 10),
                Card::new(3, 1),
                Card::new(4, 1)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(4, 10),
                Card::new(3, 1),
                Card::new(2, 1)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(4, 1),
                Card::new(3, 1),
                Card::new(3, 7),
                Card::new(4, 7)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(2, 1),
                Card::new(3, 1),
                Card::new(4, 3),
                Card::new(3, 3),
                Card::new(2, 3)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(4, 1),
                Card::new(3, 1),
                Card::new(3, 7),
                Card::new(4, 7)
            ])
            .value()
            .to_u8(),
            6
        );
        assert_eq!(
            Play::new(vec![
                Card::new(2, 1),
                Card::new(3, 1),
                Card::new(4, 3),
                Card::new(3, 3),
                Card::new(2, 3)
            ])
            .value()
            .to_u8(),
            6
        );
    }
    #[test]
    fn full_house_comparison() {
        assert!(Play::new(vec![
            Card::new(1, 4),
            Card::new(4, 4),
            Card::new(3, 7),
            Card::new(3, 7),
            Card::new(4, 7)
        ])
        .beats(&Play::new(vec![
            Card::new(2, 3),
            Card::new(3, 3),
            Card::new(4, 6),
            Card::new(3, 6),
            Card::new(2, 6)
        ])));
        assert!(Play::new(vec![
            Card::new(1, 4),
            Card::new(4, 4),
            Card::new(3, 7),
            Card::new(3, 7),
            Card::new(4, 7)
        ])
        .beats(&Play::new(vec![
            Card::new(2, 3),
            Card::new(3, 3),
            Card::new(4, 6),
            Card::new(3, 6),
            Card::new(2, 6)
        ])));
        assert_eq!(
            Play::new(vec![
                Card::new(1, 5),
                Card::new(4, 5),
                Card::new(2, 5),
                Card::new(1, 2),
                Card::new(4, 2)
            ])
            .beats(&Play::new(vec![
                Card::new(4, 11),
                Card::new(4, 11),
                Card::new(4, 9),
                Card::new(3, 9),
                Card::new(3, 9)
            ])),
            false
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 5),
                Card::new(4, 5),
                Card::new(2, 5),
                Card::new(1, 2),
                Card::new(4, 2)
            ])
            .beats(&Play::new(vec![
                Card::new(4, 11),
                Card::new(4, 11),
                Card::new(4, 9),
                Card::new(3, 9),
                Card::new(3, 9)
            ])),
            false
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(3, 10),
                Card::new(3, 1),
                Card::new(4, 1)
            ])
            .beats(&Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(4, 10),
                Card::new(3, 1),
                Card::new(2, 1)
            ])),
            false
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(3, 10),
                Card::new(3, 1),
                Card::new(4, 1)
            ])
            .beats(&Play::new(vec![
                Card::new(1, 10),
                Card::new(4, 10),
                Card::new(4, 10),
                Card::new(3, 1),
                Card::new(2, 1)
            ])),
            false
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(4, 1),
                Card::new(3, 1),
                Card::new(3, 7),
                Card::new(4, 7)
            ])
            .beats(&Play::new(vec![
                Card::new(2, 1),
                Card::new(3, 1),
                Card::new(4, 3),
                Card::new(3, 3),
                Card::new(2, 3)
            ])),
            false
        );
        assert_eq!(
            Play::new(vec![
                Card::new(1, 1),
                Card::new(4, 1),
                Card::new(3, 1),
                Card::new(3, 7),
                Card::new(4, 7)
            ])
            .beats(&Play::new(vec![
                Card::new(2, 1),
                Card::new(3, 1),
                Card::new(4, 3),
                Card::new(3, 3),
                Card::new(2, 3)
            ])),
            false
        );
    }
}
