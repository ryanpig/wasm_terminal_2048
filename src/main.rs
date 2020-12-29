mod tile;
mod game;
use crate::tile::{Direction};
use crate::game::{Game};
extern crate termion;

use std::io::{stdin};
use std::io::{stdout, Write};
use std::io::{Stdout, StdoutLock};
use std::{thread, time};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
 

mod gui { 
pub const CONTROL_TEXT: &str = "    ============= 2048 ============= \n\r
    UP-DOWN-RIGHT-LEFT: direction \n\r
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
             
    for c in stdin.keys() {
        let mut flag_start_new_game = false;
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('n') => {game.start_game(); flag_start_new_game = true;},
            Key::Right =>  game.action(Direction::Right), 
            Key::Left =>  game.action(Direction::Left), 
            Key::Up =>  game.action(Direction::Up), 
            Key::Down =>  game.action(Direction::Down), 
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

    /* test
    let two_second = time::Duration::from_millis(2000);
    loop {
        thread::sleep(two_second);
        game.action(Direction::Right);
        let r = game.next();
        if r {
            render(&mut stdout, &game);
        }
    }
    */
}


