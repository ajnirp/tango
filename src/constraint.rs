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

// Constraints are stored inside a `Board` as an array of `side*side` ints. Each
// element is an 8-bit bitmask ABCDEFGH where A through D are the bits
// representing a "x" constraint North, East, South, West, and E through H are
// the bits representing a "=" constraint North, East, South, West.
pub fn parse_constraints(constraints: &Vec<Constraint>, side: usize) -> Vec<u8> {
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
