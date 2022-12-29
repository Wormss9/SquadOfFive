use crate::Card;
mod value;
use self::value::Value;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(mut cards: Vec<Card>) -> Self {
        cards.sort_unstable_by(|a, b| b.cmp(a));
        Self { cards }
    }

    fn beats(&self,other:&Self)->bool{
        false
    }

    fn same_value(&self) -> bool {
        let first_card = &self.cards[0];
        self.cards
            .clone()
            .iter()
            .filter(|card| card.value == first_card.value)
            .count()
            == 0
    }

    fn same_color(&self) -> bool {
        let first_card = &self.cards[0];
        self.cards
            .clone()
            .iter()
            .filter(|card| card.color == first_card.color)
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

    fn full_house(&self) -> bool {
        (Hand::new(self.cards[0..2].to_vec()).value() == Value::Three
            && Hand::new(self.cards[3..5].to_vec()).value() == Value::Pair)
            || (Hand::new(self.cards[0..1].to_vec()).value() == Value::Pair
                && Hand::new(self.cards[2..5].to_vec()).value() == Value::Three)
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
                } else if self.straight() {
                    Value::Straight
                } else if self.same_color() {
                    Value::Flush
                } else if self.full_house() {
                    Value::FullHouse
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Card;

    // #[test]
    // fn eq() {
    //     let hand1 = Hand::new(vec![Card::new(0, 1), Card::new(1, 0), Card::new(1, 1)]);
    //     let hand2 = Hand::new(vec![Card::new(1, 0), Card::new(0, 1), Card::new(1, 1)]);
    //     let hand3 = Hand::new(vec![Card::new(1, 1), Card::new(1, 0), Card::new(1, 1)]);

    //     assert_eq!(hand1, hand2);
    //     assert_ne!(hand2, hand3);
    // }

    // #[test]
    // fn ord() {
    //     assert!(Card::new(1, 1) > Card::new(0, 1));
    //     assert!(Card::new(1, 1) > Card::new(1, 0));
    //     assert!(Card::new(2, 1) > Card::new(2, 0));
    //     assert!(Card::new(1, 0) > Card::new(0, 2));
    //     assert!(Card::new(1, 1) > Card::new(0, 0));
    // }
}
