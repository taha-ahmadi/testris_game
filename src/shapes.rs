
use std::{collections::HashSet, ops::Add};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Pos;
  
    fn add(self, rhs: Self) -> Self::Output {
      Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Pos> for &Shape {
    type Output = Shape;
  
    fn add(self, rhs: Pos) -> Self::Output {
      Shape {
        typ: self.typ,
        positions: self.positions.iter().map(|&pos| pos + rhs).collect(),
        anchor: self.anchor + rhs,
      }
    }
  }
  
#[derive(Debug, Clone)]
pub struct Shape {
    pub typ: &'static str,
    pub positions: HashSet<Pos>,
    pub anchor: Pos,
}

impl Shape {
    pub fn new_I() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
   pub fn new_O() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }

    pub fn new_J() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, -1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
 
    pub fn new_L() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
    pub fn new_S() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, -1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
    pub fn new_Z() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(0, -1), Pos(1, 0), Pos(1, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
    pub fn new_T() -> Self {
        Self {
            typ: "🟦",
            positions: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }

    pub fn new_random() -> Self{
        let random = (rand::random::<f64>() * 7.0).floor() as u8;
        match random{
            0 => Self::new_I(),
            1 => Self::new_O(),
            2 =>  Self::new_J(),
            3 =>  Self::new_L(),
            4 =>  Self::new_S(),
            5 =>  Self::new_Z(),
            6 =>  Self::new_T(),
            _ => unreachable!(),
        }
    }

    pub fn typ(&self) -> &'static str {
        self.typ
      }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.positions.iter().copied()
      }
    
      pub fn has_position(&self, pos: Pos) -> bool {
        self.positions.contains(&pos)
      }
    
      pub fn collides_with(&self, other: &Shape) -> bool {
        self.positions.intersection(&other.positions).count() > 0
      }
      pub fn rotated(&self) -> Self {
        let Pos(a, b) = self.anchor;
    
        Self {
          typ: self.typ,
          positions: self
            .iter_positions()
            .map(|Pos(x, y)| Pos(-y + b + a, x - a + b))
            .collect(),
          anchor: self.anchor,
        }
      } 

      pub fn remove_line(&mut self, y: i32) {
        self.positions = self
          .positions
          .iter()
          .copied()
          .filter(|pos| pos.1 != y)
          .map(|pos| {
            if pos.1 >= y {
              pos
            } else {
              Pos(pos.0, pos.1 + 1)
            }
          })
          .collect();
      }
}
