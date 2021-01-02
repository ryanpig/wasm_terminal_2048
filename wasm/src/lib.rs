
extern crate wasm_terminal_2048;
use wasm_terminal_2048::game::{Game};
use wasm_terminal_2048::tile::{Direction};

use wasm_bindgen::prelude::*;


mod gui { 
pub const CONTROL_TEXT: &str = "    ============= 2048 ============= \n\r
    UP-DOWN-RIGHT-LEFT: direction \n\r
    k-j-h-l: direction \n\r
    n: new game \n\r
    ================================ \n\r
    Current steps:  "; 
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct GameController {
    steps: u32,
    game: Game, 
}

#[wasm_bindgen]
impl GameController {

    pub fn new() -> Self {
        let game = Game::new();
        GameController {
            steps: 0,
            game: game,
        }
    }

    pub fn run(&mut self) -> String {
        self.game.start_game();
        format!("{}", self.game.get_board())
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    pub fn start_new_game(&mut self) {
       self.game.start_game();
       self.steps = 0;
    }

    pub fn next(&mut self) -> bool {
        self.steps += 1;
        self.game.next()
    }

    pub fn action(&mut self, direction: i32) {
        match direction  {
            0  => self.game.action(Direction::Left),
            1  => self.game.action(Direction::Right),
            2  => self.game.action(Direction::Up),
            3  => self.game.action(Direction::Down),
            _ => {}
        }
    }

    pub fn render(&self) -> String {
        format!("{}{}{}\n\r", 
            self.game.get_board(),
            gui::CONTROL_TEXT,
            self.game.get_steps())
    }
}

