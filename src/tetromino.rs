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
    pub fn new(width: u32, height:u32) -> Self{
        return Self{
            width: width as i32,
            height: height as i32,
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
      
      pub fn iter_positions(&self) -> impl Iterator<Item = shapes::Pos> {
        let width = self.width;
        let height = self.height;
    
        (0..height).flat_map(move |y| (0..width).map(move |x| shapes::Pos(x, y)))
      }

  pub fn get(&self, pos: shapes::Pos) -> Option<&'static str> {
    if self.current_shape.has_position(pos) {
      Some(self.current_shape.typ())
    } else {
      self
        .fixed_shapes
        .iter()
        .find(|shape| shape.has_position(pos))
        .map(|shape| shape.typ())
    }
  }

  pub fn is_colliding(&self, shape: &shapes::Shape) -> bool {
    self
      .fixed_shapes
      .iter()
      .any(|fixed_shape| fixed_shape.collides_with(shape))
  }

  pub fn is_line_full(&self, y: i32) -> bool {
    self
      .fixed_shapes
      .iter()
      .flat_map(|shape| shape.iter_positions())
      .filter(|pos| pos.1 == y)
      .collect::<HashSet<_>>()
      .len() as i32
      == self.width
  }

  fn remove_line(&mut self, y: i32) {
    for shape in self.fixed_shapes.iter_mut() {
      shape.remove_line(y);
    }
  }

  fn remove_full_lines(&mut self) {
    for y in 0..self.height {
      if self.is_line_full(y) {
        self.remove_line(y);
      }
    }
  }

  pub fn tick(&mut self) {
    if self.lost {
      return;
    }

    let translated_current_shape = &self.current_shape + shapes::Pos(0, 1);

    if self.is_out_of_bounds(&translated_current_shape)
      || self.is_colliding(&translated_current_shape)
    {
      // Make current shape fixed

      let new_fixed_shape = mem::replace(
        &mut self.current_shape,
        &shapes::Shape::new_random() + shapes::Pos((self.width - 1) / 2, 0),
      );

      self.fixed_shapes.push(new_fixed_shape);
      self.remove_full_lines();

      if self.is_colliding(&self.current_shape) {
        self.lost = true;
      }
    } else {
      self.current_shape = translated_current_shape;
    }
  }

  pub fn shift(&mut self, direction: Direction) {
    if self.lost {
      return;
    }

    let translated_current_shape = &self.current_shape
      + match direction {
        Direction::Left => shapes::Pos(-1, 0),
        Direction::Right => shapes::Pos(1, 0),
      };

    if !self.is_out_of_bounds(&translated_current_shape)
      && !self.is_colliding(&translated_current_shape)
    {
      self.current_shape = translated_current_shape;
    }
  }

  pub fn rotate(&mut self) {
    if self.lost {
      return;
    }

    let rotated_current_shape = self.current_shape.rotated();

    if !self.is_out_of_bounds(&rotated_current_shape)
      && !self.is_colliding(&rotated_current_shape)
    {
      self.current_shape = rotated_current_shape;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GameBoard;

  #[test]
  fn test() {
    let mut tetris = GameBoard::new(10, 30);
    tetris.tick();
    tetris.tick();
    tetris.tick();

    println!("{:#?}", tetris);
  }
}