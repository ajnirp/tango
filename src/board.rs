use crate::constraint::{Constraint, parse_constraints};

// Boards are stored as arrays of `side*side` ints. `side` is the number of
// in one side of the square board.
// 0 = sun, 1 = moon, 2 = blank square.
#[derive(Debug)]
pub struct Board {
    side: usize,
    cells: Vec<u8>,
    constraints: Vec<u8>,
}

impl Board {
    pub fn side(&self) -> usize {
        self.side
    }

    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn row(&self, i: usize) -> usize {
        i / self.side
    }

    pub fn col(&self, i: usize) -> usize {
        i % self.side
    }

    pub fn at(&self, r: usize, c: usize) -> u8 {
        self.cells[r*self.side + c]
    }

    pub fn at_index(&self, i: usize) -> u8 {
        self.cells[i]
    }

    pub fn set_index(&mut self, i: usize, val: u8) {
        self.cells[i] = val;
    }

    pub fn constraint_at(&self, r: usize, c: usize) -> u8 {
        self.constraints[r*self.side + c]
    }
    
    pub fn inside(&self, r: usize, c: usize) -> bool {
        r*self.side + c < self.side*self.side
    }

    pub fn row_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.cells[start..end]
    }

    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    pub fn parse_from(board: &str, constraints: &Vec<Constraint>) -> Board {
        let mut cells = Vec::<u8>::with_capacity(board.len());
        let side = board.len().isqrt();
        for c in board.chars() {
            cells.push(c.to_digit(10).unwrap() as u8);
        }
        Board {
            side: side,
            cells: cells,
            constraints: parse_constraints(&constraints, side),
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.cells() == other.cells()
    }
}
