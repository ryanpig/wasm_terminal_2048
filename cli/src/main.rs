extern crate wasm_terminal_2048;
extern crate termion;

use wasm_terminal_2048::game::{Game};
use wasm_terminal_2048::tile::{Direction};

use std::io::{stdin};
use std::io::{stdout, Write};
use std::io::{Stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
 
mod gui { 
pub const CONTROL_TEXT: &str = "    ============= 2048 ============= \n\r
    UP-DOWN-RIGHT-LEFT: direction \n\r
    k-j-h-l: direction \n\r
    n: new game \n\r
    q: quit game \n\r
    ================================ \n\r
    Current steps:  "; 
}

fn render(stdout: &mut Stdout, game: &Game) {
    write!(stdout, "{}{}{}{}{}\n {}\n\r", 
        termion::clear::All,
        termion::cursor::Goto(1,1),
        game.get_board(),
        gui::CONTROL_TEXT,
        game.get_steps(),
        termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let stdin = stdin();
    // using raw mode to accept single key input
    let mut stdout = stdout().into_raw_mode().unwrap();

    // start a new game
    let mut game = Game::new();
    game.start_game();
    render(&mut stdout, &game);
             
    // key handle loop
    for c in stdin.keys() {
        let mut flag_start_new_game = false;
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('n') => {game.start_game(); flag_start_new_game = true;},
            Key::Right | Key::Char('l') =>  game.action(Direction::Right), 
            Key::Left | Key::Char('h')=>  game.action(Direction::Left), 
            Key::Up | Key::Char('k') =>  game.action(Direction::Up), 
            Key::Down |  Key::Char('j')=>  game.action(Direction::Down), 
            _ => continue,
        }

        if flag_start_new_game { 
            render(&mut stdout, &game);
            continue;
        }

        let r = game.next();
        if r {
            render(&mut stdout, &game);
        } 
    }
}


