use std::ops::Not;

/// The two player symbols of *tic-tac-toe*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    /// The player which plays first.
    X,
    /// The second player symbol.
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

/// Possible errors returned by [`Game::set`].
///
/// [`Game::set`]: struct.Game.html#method.set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetError {
    /// One of the dimensions of `position` were greater than
    /// than the `board_size` of the given game board.
    OutOfBounds,
    /// The desired position was already set in a previous
    /// turn.
    Occupied,
    /// The length of the `position` slice is not equal to
    /// the number of dimensions.
    InvalidDimension,
    /// The given player has placed a piece in the previous turn,
    /// expected the other player.
    WrongPlayer,
    /// The game is already terminated, no more pieces can be played.
    GameFinished,
}

/// The current state of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// The game is not over yet.
    Ongoing,
    /// The game was won by the given player.
    Victory(Player),
    /// All spaces are occupied without
    /// any player winning.
    Draw,
}

/// A game of *tic-tac-toe* with a given dimension and board size.
///
/// ## Examples
///
/// An ordinary game of *tic-tac-toe*.
///
/// ```
/// use tic_tac_toe::{Game, GameState, Player};
///
/// let mut game = Game::new(2, 3);
/// // ---
/// // ---
/// // ---
/// assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 1]));
/// // ---
/// // -X-
/// // ---
/// assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 0]));
/// // O--
/// // -X-
/// // ---
/// assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[2, 0]));
/// // O--
/// // -X-
/// // X--
/// assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[0, 2]));
/// // O-O
/// // -X-
/// // X--
/// assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[0, 1]));
/// // OXO
/// // -X-
/// // X--
/// assert_eq!(Ok(GameState::Ongoing), game.set(Player::O, &[2, 2]));
/// // OXO
/// // -X-
/// // X-O
/// assert_eq!(Ok(GameState::Victory(Player::X)), game.set(Player::X, &[2, 1]));
/// // OXO
/// // -X-
/// // XXO
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    dimensions: usize,
    board_size: usize,
    states: Box<[Option<Player>]>,
    turns: usize,
    active_player: Player,
    game_state: GameState,
}

impl Game {
    /// Creates a new game of *tic-tac-toe*.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tic_tac_toe::Game;
    /// let mut game = Game::new(2, 3);
    /// # let _ = &mut game;
    /// ```
    pub fn new(dimensions: usize, board_size: usize) -> Self {
        assert_ne!(board_size, 0);
        assert_ne!(dimensions, 0);

        let state_count = board_size.pow(dimensions as u32);
        let states = vec![None; state_count].into_boxed_slice();

        Self {
            dimensions,
            board_size,
            states,
            turns: 0,
            active_player: Player::X,
            game_state: GameState::Ongoing,
        }
    }

    /// Returns the current state of the game.
    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    /// Returns how many pieces were already placed
    /// on the board.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tic_tac_toe::{Game, Player};
    /// let mut game = Game::new(2, 3);
    ///
    /// assert_eq!(0, game.turns());
    /// game.set(Player::X, &[1, 1]).unwrap();
    /// assert_eq!(1, game.turns());
    /// ```
    pub fn turns(&self) -> usize {
        self.turns
    }

    /// Returns the player which may set the next piece on the board.
    ///
    /// The return value of this function is unspecified in case the game is already over.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use tic_tac_toe::{Game, Player};
    /// let mut game = Game::new(2, 3);
    ///
    /// assert_eq!(Player::X, game.active_player());
    /// game.set(Player::X, &[1, 1]).unwrap();
    /// assert_eq!(Player::O, game.active_player());
    /// ```
    pub fn active_player(&self) -> Player {
        self.active_player
    }

    fn at(&self, position: &[usize]) -> Option<Player> {
        self.states[self.idx(position)]
    }

    fn idx(&self, position: &[usize]) -> usize {
        position.iter().enumerate().fold(0, |state, (idx, p)| {
            state + self.board_size.pow(idx as u32) * p
        })
    }

    fn calculate_state(&self, player: Player, position: &[usize]) -> GameState {
        for dim in 0..self.dimensions {
            'other: for others in 0..3usize.pow(dim as u32) {
                let mut pos = position.to_owned();
                for i in 0..self.board_size {
                    pos[dim] = i;

                    let mut sub_dim = 0;
                    let mut others = others;
                    while others != 0 {
                        match others % 3 {
                            1 => pos[sub_dim] = i,
                            2 => pos[sub_dim] = self.board_size - 1 - i,
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

    /// Puts a piece at the given `position`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tic_tac_toe::{Game, GameState, Player, SetError};
    ///
    /// let mut game = Game::new(2, 3);
    ///
    /// assert_eq!(Ok(GameState::Ongoing), game.set(Player::X, &[1, 1]));
    /// assert_eq!(Err(SetError::Occupied), game.set(Player::O, &[1,1 ]));
    /// ```
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

        if !position.iter().all(|&p| p < self.board_size) {
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
