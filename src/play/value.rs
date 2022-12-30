#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    None = 0,
    Single = 1,
    Pair = 2,
    Three = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    StraightFlush = 7,
    FourOfAKind = 8,
    FiveOfAKind = 9,
    SixOfAKind = 10,
    SevenOfAKind = 11,
}
impl Value {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Single => 1,
            Self::Pair => 2,
            Self::Three => 3,
            Self::Straight => 4,
            Self::Flush => 5,
            Self::FullHouse => 6,
            Self::StraightFlush => 7,
            Self::FourOfAKind => 8,
            Self::FiveOfAKind => 9,
            Self::SixOfAKind => 10,
            Self::SevenOfAKind => 11,
        }
    }
}
