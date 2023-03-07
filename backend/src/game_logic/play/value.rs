#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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
