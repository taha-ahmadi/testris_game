use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    pub positions: HashSet<Pos>,
    pub anchor: Pos,
}

impl Shape {
    pub fn new_I() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
   pub fn new_O() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }

    pub fn new_J() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, -1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
 
    pub fn new_L() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
    pub fn new_S() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, -1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
    pub fn new_Z() -> Self {
        Self {
            positions: [Pos(0, 0), Pos(0, -1), Pos(1, 0), Pos(1, 1)]
                .into_iter()
                .collect(),
            anchor: Pos(0, 1),
        }
    }
    pub fn new_T() -> Self {
        Self {
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

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.positions.iter().copied()
      }
    
      pub fn has_position(&self, pos: Pos) -> bool {
        self.positions.contains(&pos)
      }
    
      pub fn collides_with(&self, other: &Shape) -> bool {
        self.positions.intersection(&other.positions).count() > 0
      }
    
}
