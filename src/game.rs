use crate::tile::{Board, Direction};

/// wasm-terminal-2048
///
/// wasm-terminal-2048 library  is composed of `tile` module and `game` module. The `tile` has `Board`
/// struct that store 16 tile values and can move tiles toward four directions. The other `game`
/// struct includes `Game` struct that is responsible for high-level logic that is the bridge between the front-end, such as a command line terminal or a browser application, and the back-end game logic. 
///
/// `Game` is used to control game iterations, execute user actions and update internal tile values in the board. The example is as below. The complete example can be found in `cli/src/main.rs` for command line terminal or `wasm/src/lib.rs` for webassembly
///
/// ```ignore
/// 
///  // start a new game
///  let mut game = Game::new();
///  game.start_game();
///  // call render function depends on the front-end
///  render()
///
///  // looping: process user input and execute an action 
///  for c in stdin.keys() {
///    match c.unwrap() {
///      Key::Down |  Key::Char('j') => game.action(Direction::Down), 
///      _ => continue,
///    }
///
///    // move to next iteration, and render the updated board
///    game.next();
///    render(&mut stdout, &game);
///  }
///
/// ```

pub struct Game {
    board: Board,
    steps: u32,
}

impl Game {

    /// Create a new board with the default tile values 0
    pub fn new() -> Self {
        let board: Board = Default::default();
        Self {
            board: board,
            steps: 0
        }
    }

    /// Create a new board by a given tile values  (Only for testing)
    ///
    /// # Arguments
    /// * `values` Tile values
    ///
    #[cfg(test)]
    fn new_with_values(values: Vec<u16>) -> Self {
        Self {
            board: Board::new(values),
            steps: 0,
        }
    }

    /// This method resets the internal tile values, and start a new game
    pub fn start_game(&mut self) {
        self.reset_game();
        self.board.generate_new_tile();
    }
    

    /// By a given direction, updates the internal tile values after the movement
    ///
    /// # Arguments
    /// * `Direction` The direction of tile movement  
    ///
    pub fn action(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.board.move_up(),
            Direction::Down => self.board.move_down(), 
            Direction::Left => self.board.move_left(), 
            Direction::Right => self.board.move_right(), 
        }
    }

    /// Get the internal Board 
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Get the steps of the current game 
    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    /// Generate a new tile and increment steps. If return values is false, it means 
    /// there is no empty tile for new tile generation.
    pub fn next(&mut self) -> bool {
        let r = self.board.generate_new_tile();
        self.steps += if r {1} else {0};
        r
    }

    /// Reset the game steps, and internal board (all tile values are 0)
    fn reset_game(&mut self) {
        self.steps = 0;
        self.board.reset();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_game() {
        let mut game = Game::new();
        game.start_game();
    }

    #[test]
    fn test_actions() {
        let int_values = vec![
                0,0,0,0,
                0,2,0,0,
                0,0,0,0,
                0,0,0,0,
            ];
        let left_expected_values = vec![
                0,0,0,0,
                2,0,0,0,
                0,0,0,0,
                0,0,0,0,
            ];

        let right_expected_values = vec![
                0,0,0,0,
                0,0,0,2,
                0,0,0,0,
                0,0,0,0,
            ];

        let up_expected_values = vec![
                0,2,0,0,
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
            ];
        let down_expected_values = vec![
                0,0,0,0,
                0,0,0,0,
                0,0,0,0,
                0,2,0,0,
            ];
        let mut game = Game::new_with_values(int_values.clone());
        game.action(Direction::Left);
        assert_eq!(game.board.get_values().to_vec(), left_expected_values);
        let mut game = Game::new_with_values(int_values.clone());
        game.action(Direction::Right);
        assert_eq!(game.board.get_values().to_vec(), right_expected_values);
        let mut game = Game::new_with_values(int_values.clone());
        game.action(Direction::Up);
        assert_eq!(game.board.get_values().to_vec(), up_expected_values);
        let mut game = Game::new_with_values(int_values.clone());
        game.action(Direction::Down);
        assert_eq!(game.board.get_values().to_vec(), down_expected_values);
    }
}
