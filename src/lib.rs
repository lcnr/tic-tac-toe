use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetError {
    OutOfBounds,
    Occupied,
    InvalidDimension,
    WrongPlayer,
    GameFinished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Victory(Player),
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub dimensions: usize,
    pub field_size: usize,
    pub states: Box<[Option<Player>]>,
    pub turns: usize,
    pub active_player: Player,
    pub game_state: GameState,
}

impl Game {
    pub fn new(dimensions: usize, field_size: usize) -> Self {
        assert_ne!(field_size, 0);
        assert_ne!(dimensions, 0);

        let state_count = field_size.pow(dimensions as u32);
        let states = vec![None; state_count].into_boxed_slice();

        Self {
            dimensions,
            field_size,
            states,
            turns: 0,
            active_player: Player::X,
            game_state: GameState::Ongoing,
        }
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    fn at(&self, position: &[usize]) -> Option<Player> {
        self.states[self.idx(position)]
    }

    fn idx(&self, position: &[usize]) -> usize {
        position.iter().enumerate().fold(0, |state, (idx, p)| {
            state + self.field_size.pow(idx as u32) * p
        })
    }

    fn calculate_state(&self, player: Player, position: &[usize]) -> GameState {
        for dim in 0..self.dimensions {
            'other: for others in 0..3usize.pow(dim as u32) {
                let mut pos = position.to_owned();
                for i in 0..self.field_size {
                    pos[dim] = i;

                    let mut sub_dim = 0;
                    let mut others = others;
                    while others != 0 {
                        match others % 3 {
                            1 => pos[sub_dim] = i,
                            2 => pos[sub_dim] = self.field_size - 1 - i,
                            _ => (),
                        }

                        sub_dim += 1;
                        others = others / 3;
                    }

                    if self.at(&pos) != Some(player) {
                        continue 'other;
                    }
                }
                
                return GameState::Victory(player);
            }
        }

        if self.turns == self.states.len() {
            GameState::Draw
        } else {
            GameState::Ongoing
        }
    }

    /// Sets a piece at the given `position`.
    ///
    /// Returns an error if the `position` is already occupied or
    /// out of bounds.
    pub fn set(&mut self, player: Player, position: &[usize]) -> Result<GameState, SetError> {
        if self.game_state != GameState::Ongoing {
            return Err(SetError::GameFinished);
        }

        if player != self.active_player {
            return Err(SetError::WrongPlayer);
        }

        if position.len() != self.dimensions {
            return Err(SetError::InvalidDimension);
        }

        if !position.iter().all(|&p| p < self.field_size) {
            return Err(SetError::OutOfBounds);
        }

        let idx = self.idx(position);

        if self.states[idx].is_some() {
            return Err(SetError::Occupied);
        }

        self.states[idx] = Some(player);
        self.turns += 1;

        self.game_state = self.calculate_state(player, position);

        self.active_player = !player;
        Ok(self.game_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_err() {
        let mut game = Game::new(2, 3);
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 1]));

        let clone = game.clone();
        assert_eq!(Err(SetError::WrongPlayer), game.set(Player::X, &[0, 0]));
        assert_eq!(
            Err(SetError::InvalidDimension),
            game.set(Player::O, &[0, 0, 1])
        );
        assert_eq!(Err(SetError::OutOfBounds), game.set(Player::O, &[3, 1]));
        assert_eq!(Err(SetError::Occupied), game.set(Player::O, &[1, 1]));
        assert_eq!(clone, game);
    }

    #[test]
    fn game3x3() {
        let mut game = Game::new(2, 3);
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[2, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[0, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 2]));
        assert_eq!(
            Ok(GameState::Victory(Player::X)),
            game.set(Player::X, &[1, 2])
        );
        assert_eq!(GameState::Victory(Player::X), game.game_state());
    }

    #[test]
    fn game3x3x3() {
        let mut game = Game::new(3, 3);
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 1, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 2, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 0, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[1, 2, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[0, 2, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 0, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[2, 1, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 1, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[0, 0, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 2, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[2, 2, 1]));
        assert_eq!(
            Ok(GameState::Victory(Player::O)),
            game.set(Player::O, &[2, 1, 1])
        );
        assert_eq!(GameState::Victory(Player::O), game.game_state());
        assert_eq!(Err(SetError::GameFinished), game.set(Player::X, &[0, 0, 0]));
    }

    #[test]
    fn game4x4x4() {
        let mut game = Game::new(3, 4);
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 1, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 2, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 2, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[1, 2, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 0, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[1, 3, 3]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[0, 0, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 0, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[2, 1, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 3, 3]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[2, 3, 3]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 3, 0]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[0, 3, 1]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 1, 2]));
        assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 3, 1]));
        assert_eq!(
            Ok(GameState::Victory(Player::O)),
            game.set(Player::O, &[3, 0, 3])
        );
        assert_eq!(GameState::Victory(Player::O), game.game_state());
        assert_eq!(Err(SetError::GameFinished), game.set(Player::X, &[0, 0, 0]));
    }
}
