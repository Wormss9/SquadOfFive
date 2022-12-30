use std::cmp::Ordering;

mod color;
mod value;

use color::Color;
use value::Value;

#[derive(Debug, Clone)]
pub struct Card {
    pub value: Value,
    pub color: Color,
}

impl Card {
    pub fn new(color: u8, value: u8) -> Self {
        Self {
            value: Value::from_u8(value),
            color: Color::from_u8(color),
        }
    }
    pub fn to_path(&self) -> String {
        format!("{:}{:02}", self.color.to_char(), self.value.to_u8())
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
        assert_eq!(Card::new(1, 1), Card::new(1, 1));
        assert_ne!(Card::new(1, 1), Card::new(2, 1));
        assert_ne!(Card::new(1, 1), Card::new(1, 2));
    }

    #[test]
    fn ord() {
        assert!(Card::new(2, 2) > Card::new(2, 1));
        assert!(Card::new(2, 2) > Card::new(1, 2));
        assert!(Card::new(2, 3) > Card::new(1, 3));
        assert!(Card::new(1, 2) > Card::new(3, 1));
        assert!(Card::new(2, 2) > Card::new(1, 1));
    }

    #[test]
    fn path() {
        assert_eq!(Card::new(1, 1).to_path(), "r01".to_string());
        assert_eq!(Card::new(2, 5).to_path(), "g05".to_string());
        assert_eq!(Card::new(3, 11).to_path(), "b11".to_string());
        assert_eq!(Card::new(4, 1).to_path(), "w01".to_string());
    }
}
