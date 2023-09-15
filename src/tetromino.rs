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
