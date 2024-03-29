use std::cmp::Ordering;

mod color;
mod value;

use color::Color;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql};
use value::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub value: Value,
    pub color: Color,
}

impl Card {
    pub fn new(value: u8, color: u8) -> Self {
        Self {
            value: Value::from_u8(value),
            color: Color::from_u8(color),
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

impl ToSql for Card {
    fn to_sql(
        &self,
        _: &tokio_postgres::types::Type,
        w: &mut tokio_postgres::types::private::BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        match serde_json::to_vec(self) {
            Ok(c) => {
                w.extend_from_slice(&c);
                Ok(IsNull::No)
            }
            Err(x) => {
                eprint!("Card: {}", x);
                Ok(IsNull::Yes)
            }
        }
    }

    accepts!(JSON);
    to_sql_checked!();
}

impl FromSql<'_> for Card {
    fn from_sql<'a>(
        _: &tokio_postgres::types::Type,
        raw: &'_ [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        serde_json::from_slice(raw).map_err(Box::from)
    }

    accepts!(JSON);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eq() {
        assert_eq!(Card::new(1, 1), Card::new(1, 1));
        assert_ne!(Card::new(1, 1), Card::new(1, 2));
        assert_ne!(Card::new(1, 1), Card::new(2, 1));
        assert_eq!(Card::new(2, 3), Card::new(2, 3));
    }

    #[test]
    fn ord() {
        assert!(Card::new(2, 2) > Card::new(1, 2));
        assert!(Card::new(2, 2) > Card::new(2, 1));
        assert!(Card::new(3, 2) > Card::new(3, 1));
        assert!(Card::new(2, 1) > Card::new(1, 3));
        assert!(Card::new(2, 2) > Card::new(1, 1));
    }
}
