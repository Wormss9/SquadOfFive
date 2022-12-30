use rand::{prelude::Distribution, Rng, distributions::Standard};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl TurnState{
    pub fn random()->Self{
        let player: TurnState = rand::random();
        player
    }
}

impl Distribution<TurnState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TurnState {
        match rng.gen_range(0..3) {
            0 => TurnState::Player1,
            1 => TurnState::Player2,
            2 => TurnState::Player3,
            _ => TurnState::Player4,
        }
    }
}