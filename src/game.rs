use crate::tile::{Board, Direction};

pub struct Game {
    board: Board,
    steps: u32,
}

impl Game {
    pub fn new() -> Self {
        let board: Board = Default::default();
        Self {
            board: board,
            steps: 0
        }
    }

    #[cfg(test)]
    fn new_with_values(values: Vec<u16>) -> Self {
        Self {
            board: Board::new(values),
            steps: 0,
        }
    }

    pub fn start_game(&mut self) {
        self.reset_game();
        self.board.generate_new_tile();
    }
    

    pub fn action(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.board.move_up(),
            Direction::Down => self.board.move_down(), 
            Direction::Left => self.board.move_left(), 
            Direction::Right => self.board.move_right(), 
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    pub fn next(&mut self) -> bool {
        let r = self.board.generate_new_tile();
        self.steps += if r {1} else {0};
        r
    }

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
