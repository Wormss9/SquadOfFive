use std::cmp::Ordering;

mod value;
mod color;

use value::Value;
use color::Color;

#[derive(Debug, Clone)]
pub struct Card {
    pub value: Value,
    pub color: Color,
}

impl Card {
    pub fn new(value: u8, color: Color) -> Self {
        Self {
            value: Value::from_u8(value),
            color,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.color == other.color
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let value_comparison = self.value.cmp(&other.value);
        if value_comparison == Ordering::Equal {
            self.color.cmp(&other.color)
        } else {
            value_comparison
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eq() {
        assert_eq!(Card::new(1, Color::Red), Card::new(1, Color::Red));
        assert_ne!(Card::new(1, Color::Red), Card::new(2, Color::Red));
        assert_ne!(Card::new(1, Color::Red), Card::new(1, Color::Green));
    }

    #[test]
    fn ord() {
        assert!(Card::new(2, Color::Green) > Card::new(1, Color::Green));
        assert!(Card::new(2, Color::Green) > Card::new(2, Color::Red));
        assert!(Card::new(3, Color::Green) > Card::new(3, Color::Red));
        assert!(Card::new(2, Color::Red) > Card::new(1, Color::Blue));
        assert!(Card::new(2, Color::Green) > Card::new(1, Color::Red));
    }
}
