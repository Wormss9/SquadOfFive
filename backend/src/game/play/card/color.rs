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
            1 => Self::Red,
            2 => Self::Green,
            3 => Self::Blue,
            4 => Self::White,
            _ => panic!("Max value is 4"),
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
