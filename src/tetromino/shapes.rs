use std::collections::HashSet;

pub struct Pos (pub i32, pub i32);
pub struct Shape {
    pub position: HashSet<Pos>
}
pub const I_TETROMINO: [[char; 4]; 4] = [
    [' ', ' ', ' ', ' '],
    ['X', 'X', 'X', 'X'],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];

pub const J_TETROMINO: [[char; 4]; 4] = [
    ['X', ' ', ' ', ' '],
    ['X', 'X', 'X', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];

pub const L_TETROMINO: [[char; 4]; 4] = [
    [' ', ' ', 'X', ' '],
    ['X', 'X', 'X', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];


pub const O_TETROMINO: [[char; 4]; 4] = [
    ['X', 'X', ' ', ' '],
    ['X', 'X', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];


pub const S_TETROMINO: [[char; 4]; 4] = [
    [' ', 'X', 'X', ' '],
    ['X', 'X', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];


pub const T_TETROMINO: [[char; 4]; 4] = [
    [' ', 'X', ' ', ' '],
    ['X', 'X', 'X', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];


pub const Z_TETROMINO: [[char; 4]; 4] = [
    ['X', 'X', ' ', ' '],
    [' ', 'X', 'X', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];
