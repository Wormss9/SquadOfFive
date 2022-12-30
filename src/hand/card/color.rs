#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
    White = 3,
}

impl Color {
    pub fn from_u8(color: u8) -> Self {
        match color {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::White,
            _ => panic!("Max value is 3"),
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Self::Red => 'r',
            Self::Green => 'g',
            Self::Blue => 'b',
            Self::White => 'w',
        }
    }
}
