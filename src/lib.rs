mod game;
pub use crate::game::{Game, GameState};

pub trait Player: Eq {
    fn is_maximizer(&self) -> bool;
    fn next(&self) -> Self;
}
