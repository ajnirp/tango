// Utilities to test the solver.

use crate::board::Board;
use crate::constraint::Constraint;

pub struct Testcase {
    pub board: &'static str,
    pub constraints: Vec<Constraint>,
    pub solution: &'static str,
}

fn valid_line(line: &[u8]) -> bool {
    let side = line.len();
    let num_suns = line.into_iter().filter(|x| **x == 0).count();
    let num_moons = line.into_iter().filter(|x| **x == 1).count();
    if num_suns != side / 2 || num_moons != side / 2 {
        return false;
    }
    for i in 0..(side - 2) {
        if line[i] == line[i + 1] && line[i] == line[i + 2] {
            return false;
        }
    }
    true
}

fn valid(board: &Board, constraints: &Vec<Constraint>) -> bool {
    let side = board.side();
    for r in 0..side {
        let slice: &[u8] = board.row_slice(r * side, r * side + side);
        if !valid_line(&slice) {
            return false;
        }
    }
    for c in 0..side {
        let mut line = vec![0u8; side];
        for r in 0..side {
            line[r] = board.at(r, c);
        }
        if !valid_line(&line) {
            return false;
        }
    }
    for constraint in constraints.iter() {
        let from = board.at_index(constraint.from);
        let to = board.at_index(constraint.to);
        if constraint.eq && from == to {
            continue;
        } else if !constraint.eq && from != to {
            continue;
        }
        return false;
    }
    true
}

pub fn is_solved(board: &Board, constraints: &Vec<Constraint>) -> bool {
    valid(&board, &constraints) && board.cells().iter().all(|&x| x != 2)
}
