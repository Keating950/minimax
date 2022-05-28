use crate::Player;

pub trait Game: Sized {
    type GamePlayer: Player;
    type Move;

    fn children<'a>(
        &'a self,
        player: &'a Self::GamePlayer,
    ) -> Box<dyn Iterator<Item = (Self::Move, Self)> + '_>;

    fn state(&self) -> GameState<Self::GamePlayer>;

    fn minimax(&self, player: Self::GamePlayer) -> (Option<Self::Move>, f32) {
        match self.state() {
            GameState::Won(tok) => {
                if tok == player {
                    return (None, if player.is_maximizer() { 1. } else { -1. });
                } else {
                    return (None, if player.is_maximizer() { -1. } else { 1. });
                }
            }
            GameState::Drawn => return (None, 0.),
            GameState::Ongoing => (),
        };
        if player.is_maximizer() {
            let mut max_eval = f32::NEG_INFINITY;
            let mut best_move: Option<Self::Move> = None;
            for (pos, child) in self.children(&player) {
                let current_eval = child.minimax(player.next()).1;
                if current_eval > max_eval {
                    max_eval = current_eval;
                    best_move = Some(pos);
                }
            }
            (best_move, max_eval)
        } else {
            let mut min_eval = f32::INFINITY;
            let mut best_move: Option<Self::Move> = None;
            for (pos, child) in self.children(&player) {
                let current_eval = child.minimax(player.next()).1;
                if current_eval < min_eval {
                    min_eval = current_eval;
                    best_move = Some(pos);
                }
            }
            (best_move, min_eval)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState<T: Player> {
    Won(T),
    Drawn,
    Ongoing,
}
