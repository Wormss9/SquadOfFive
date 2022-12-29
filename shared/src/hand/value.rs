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
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Diamond => 11,
        }
    }
}
