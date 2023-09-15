use crate::shapes;
use std::{collections::HashSet, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Left,
  Right,
}

#[derive(Debug, Clone)]
pub struct GameBoard{
    pub width: i32,
    pub height: i32,
    pub current_shape: shapes::Shape,
    pub fixed_shapes: Vec<shapes::Shape>,
    lost: bool,
}

impl GameBoard{
    pub fn new(width: i32, height:i32) -> Self{
        return Self{
            width,
            height,
            current_shape: shapes::Shape::new_random(),
            fixed_shapes: vec![],
            lost: false,
        };
    }

    pub fn is_out_of_bounds(&self, shape: &shapes::Shape) -> bool {
        !shape.iter_positions().all(|pos| {
          0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height
        })
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
