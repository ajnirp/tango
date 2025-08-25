use crate::board::{col, row};

// Constraints are stored as an array of `side*side` ints. Each element is an
// 8-bit bitmask ABCDEFGH where A through D are the bits representing a "x"
// constraint North, East, South, West, and E through H are the bits
// representing a "=" constraint North, East, South, West.

pub struct Constraint {
    pub from: usize,
    pub to: usize,
    pub eq: bool,
}

// 1: j is west of i -> i's shift is 3, j's is 1
// -1: j is east of i -> i's shift is 1, j's is 3
// side: j is north of i -> i's shift is 0, j's is 2
// -side: j is south of i -> i's shift is 2, j's is 0
// any other number: not adjacent
fn adjacent(i: usize, j: usize, side: usize) -> i8 {
    let mut result = 0i8;
    if row(i, side) == row(j, side) || col(i, side) == col(j, side) {
        result = (i as i8) - (j as i8);
    }
    if result == 1 || result == -1 || result == (side as i8) || result == -(side as i8) {
        result
    } else {
        0 // not adjacent
    }
}

// Expects `result` to already have the right size.
pub fn parse_constraints(constraints: &Vec<Constraint>, result: &mut Vec<u8>, side: usize) {
    let adjacencies = [1i8, -1, side as i8, -(side as i8)];
    let from_shifts = [3u8, 1, 0, 2];
    let to_shifts = [1u8, 3, 2, 0];
    for constraint in constraints.iter() {
        let adjacency = adjacent(constraint.from, constraint.to, side);
        if adjacency == 0 {
            continue;
        }
        let index = adjacencies
            .iter()
            .position(|&x| x == adjacency)
            .unwrap_or(255) as usize;
        if index == 255 {
            continue;
        }
        let mut from_shift = from_shifts[index];
        let mut to_shift = to_shifts[index];
        if !constraint.eq {
            from_shift += 4;
            to_shift += 4;
        }
        result[constraint.from] |= 1 << from_shift;
        result[constraint.to] |= 1 << to_shift;
    }
}
