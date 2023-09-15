extern crate termion;

use std::io::{self, Write};
use std::vec;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

fn main(){
    let mut game_board: Vec<Vec<char>> = vec![vec![' '; BOARD_WIDTH]; BOARD_HEIGHT];



    let stdout = io::stdout();
    // ToDo: Handle unwrap creacefully without panic
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    print_board(&game_board);
    for c in io::stdin().keys() {
        // Handle user input here

        // Update game state here

        // Render the game here

        // Check for game over here

        // Exit game loop on game over
        if true{
            break;
        }
    }
}
fn print_board(board: &Vec<Vec<char>>) {
    for row in board.iter() {
        let row_str: String = row.iter().collect();
        println!("*");
    }
}   