extern crate termion;

use std::io;
use std::vec;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
mod tetromino;
use termion::event::Key;


const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

fn main(){
    let mut game_board: Vec<Vec<char>> = vec![vec![' '; BOARD_WIDTH]; BOARD_HEIGHT];



    let stdout = io::stdout();
    // ToDo: Handle unwrap creacefully without panic
    let mut stdout = stdout.lock().into_raw_mode().unwrap();




    // for c in io::stdin().keys() {
        
    // }


}

