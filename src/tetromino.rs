use std::vec;

use rand::Rng;



use crate::shapes;

#[derive(Debug, Clone)]
pub struct GameBoard{
    pub width: usize,
    pub height: usize,
    pub current_shape: shapes::Shape,
    pub fixed_shapes: Vec<shapes::Shape>
}

impl GameBoard{
    pub fn new(width: usize, height:usize) -> Self{
        return Self{
            width,
            height,
            current_shape: shapes::Shape::new_random(),
            fixed_shapes: vec![],
        };
    }
}

#[cfg(test)]
mod tests{
    use super::GameBoard;

    #[test]
    fn test(){
        println!("{:#?}", GameBoard::new(5, 10))
    }
}

// pub fn rotate(tetromino: &[[char; 4]; 4]) -> [[char; 4]; 4] {
//     let mut rotated_tetromino = [[' '; 4]; 4];

//     for i in 0..4 {
//         for j in 0..4 {
//             rotated_tetromino[i][j] = tetromino[3 - j][i];
//         }
//         print_2d_array(rotated_tetromino)
//     }

//     rotated_tetromino
// }


// pub fn print_2d_array(tetromino: [[char; 4]; 4]) {
//     for row in tetromino.iter() {
//         for &cell in row.iter() {
//             print!("{}", cell);
//         }
//         println!();
//     }
// }
