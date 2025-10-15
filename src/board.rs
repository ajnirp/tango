use crate::constraint::{Constraint, parse_constraints};

// Boards are stored as arrays of `side*side` ints. `side` is the number of
// in one side of the square board.
// 0 = sun, 1 = moon, 2 = blank square.
#[derive(Debug)]
pub struct Board {
    side: usize,
    cells: Vec<u8>,
    constraints: Vec<u8>,
    solve_state: SolveState,
}

#[derive(Debug)]
pub struct SolveState {
    // Vector of initially unsolved cells. Never changes.
    initially_unsolved: Vec<usize>,

    // Index into unsolved cells. We advance the index when that cell is solved.
    // When `num_solved` equals `initially_unsolved.len()`, the board is solved.
    num_solved: usize,
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

    pub fn is_solved(&self) -> bool {
        return self.solve_state.num_solved == self.solve_state.initially_unsolved.len();
    }

    pub fn next_unsolved(&self) -> Option<usize> {
        if self.is_solved() {
            None
        } else {
            Some(self.solve_state.initially_unsolved[self.solve_state.num_solved])
        }
    }

    pub fn mark_solved(&mut self) {
        // Should not increment past `initially_unsolved.len()`
        if self.solve_state.num_solved < self.solve_state.initially_unsolved.len() {
            self.solve_state.num_solved += 1;
        }
    }

    pub fn mark_unsolved(&mut self) {
        // Should not decrement past `0`.
        if self.solve_state.num_solved > 0 {
            self.solve_state.num_solved -= 1;
        }
    }

    pub fn parse_from(board: &str, constraints: &Vec<Constraint>) -> Board {
        let mut cells = Vec::<u8>::with_capacity(board.len());
        let side = board.len().isqrt();
        for c in board.chars() {
            cells.push(c.to_digit(10).unwrap() as u8);
        }

        let mut initially_unsolved = Vec::<usize>::with_capacity(board.len());
        for (index, cell) in cells.iter().enumerate() {
            if *cell == 2 {
                initially_unsolved.push(index);
            }
        }
        Board {
            side: side,
            cells: cells,
            constraints: parse_constraints(&constraints, side),
            solve_state: SolveState {
                initially_unsolved: initially_unsolved,
                num_solved: 0,
            }
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.cells() == other.cells()
    }
}
