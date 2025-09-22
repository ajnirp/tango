pub struct Constraint {
    pub from: usize,
    pub to: usize,
    pub eq: bool,
}

// 1: j is west of i -> i's shift is 3, j's is 1
// -1: j is east of i -> i's shift is 1, j's is 3
// side: j is north of i -> i's shift is 0, j's is 2
// -side: j is south of i -> i's shift is 2, j's is 0
fn adjacent(i: usize, j: usize, side: usize) -> Option<i8> {
    let mut result = 0i8;
    if i / side == j / side || i % side == j % side {
        result = (i as i8) - (j as i8);
    }
    if result == 1 || result == -1 || result == (side as i8) || result == -(side as i8) {
        Some(result)
    } else {
        None // not adjacent
    }
}

fn parse_constraints(constraints: &Vec<Constraint>, side: usize) -> Vec<u8> {
    let adjacencies = [1i8, -1, side as i8, -(side as i8)];
    let from_shifts = [3u8, 1, 0, 2];
    let to_shifts = [1u8, 3, 2, 0];
    let mut result = vec![0; side*side];
    for constraint in constraints.iter() {
        let adjacency = adjacent(constraint.from, constraint.to, side);
        if adjacency.is_none() {
            continue;
        }
        let index = adjacencies
            .iter()
            .position(|&x| x == adjacency.unwrap());
        if index.is_none() {
            continue;
        }
        let index = index.unwrap();
        let mut from_shift = from_shifts[index];
        let mut to_shift = to_shifts[index];
        if !constraint.eq {
            from_shift += 4;
            to_shift += 4;
        }
        result[constraint.from] |= 1 << from_shift;
        result[constraint.to] |= 1 << to_shift;
    }
    result
}

// Boards are stored as arrays of `side*side` ints. `side` is the number of
// in one side of the square board.
// 0 = sun, 1 = moon, 2 = blank square.
#[derive(Debug)]
pub struct Board {
    side: usize,
    cells: Vec<u8>,

    // Constraints are stored as an array of `side*side` ints. Each element is an
    // 8-bit bitmask ABCDEFGH where A through D are the bits representing a "x"
    // constraint North, East, South, West, and E through H are the bits
    // representing a "=" constraint North, East, South, West.
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
