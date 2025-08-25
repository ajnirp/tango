// Utilities to test the solver.

use crate::board::{at, side};
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

fn valid(board: &Vec<u8>, constraints: &Vec<Constraint>) -> bool {
    let side = side(&board);
    for r in 0..side {
        let slice: &[u8] = &board[(r * side)..(r * side + side)];
        if !valid_line(&slice) {
            return false;
        }
    }
    for c in 0..side {
        let mut line = vec![0u8; side];
        for r in 0..side {
            line[r] = at(board, r, c);
        }
        if !valid_line(&line) {
            return false;
        }
    }
    for constraint in constraints.iter() {
        let from = board[constraint.from];
        let to = board[constraint.to];
        if constraint.eq && from == to {
            continue;
        } else if !constraint.eq && from != to {
            continue;
        }
        return false;
    }
    true
}

pub fn is_solved(board: &Vec<u8>, constraints: &Vec<Constraint>) -> bool {
    valid(&board, &constraints) && board.iter().all(|&x| x != 2)
}
