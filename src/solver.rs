use crate::board::{at, col, inside, row, side};

fn can_set(board: &Vec<u8>, i: usize, constraints: &Vec<u8>, new: u8) -> bool {
    let (drs, dcs) = ([0usize, 1], [1usize, 0]);
    let side = side(&board);
    let (r, c) = (row(i, side), col(i, side));

    for v in 0..2 {
        let (dr, dc) = (drs[v], dcs[v]);
        let (mut _r, mut _c) = (r * dc, c * dr);

        // `one` = the value two indices before `curr`
        // `two` = the value one index before `curr`
        let (mut one, mut two) = (255u8, 255u8);

        // number of cells in the row/col that are already equal to `new`
        let mut num_existing = 0;

        for _ in 0..side {
            let curr = if (_r, _c) == (r, c) { new } else { at(board, _r, _c) };

            // three consecutive identical not allowed
            if one == two && two == curr && curr != 2 {
                return false;
            }
            one = two;
            two = curr;

            if curr == new { num_existing += 1; }

            _r += dr;
            _c += dc;
        }
        // no more than `side / 2` in a row or column
        // for example, for a 8x8 board, no more than 4 in a row or column
        if num_existing > side / 2 {
            return false;
        }
    }

    // abide by constraints
    // north, east, south, west
    let _drs = [-1i16, 0, 1, 0];
    let _dcs = [0i16, 1, 0, -1];
    let constraint = at(constraints, r, c);
    for j in 0..8 {
        if constraint & (1 << j) == 0 { continue; }

        let _nr = (r as i16) + _drs[j % 4];
        let _nc = (c as i16) + _dcs[j % 4];
        if _nr < 0 || _nc < 0 { continue; }

        let (nr, nc) = (_nr as usize, _nc as usize);
        if !inside(&board, nr, nc) { continue; }

        let nbr_val = at(board, nr, nc);
        if nbr_val == 2 { continue; }

        if (j < 4 && nbr_val == new) || (j >= 4 && nbr_val != new) {
            continue;
        }
        return false;
    }

    true
}

fn helper(board: &mut Vec<u8>, i: usize, constraints: &Vec<u8>) -> bool {
    if i == board.len() {
        return true;
    }
    if board[i] != 2 {
        // already filled, move on
        return helper(board, i + 1, constraints);
    }
    for new in 0..2 {
        // set a value only if it's safe to do so
        if can_set(board, i, constraints, new) {
            board[i] = new;
            if helper(board, i + 1, constraints) {
                return true;
            }
            board[i] = 2;
        }
    }
    false
}

pub fn solve(board: &mut Vec<u8>, constraints: &Vec<u8>) -> bool {
    helper(board, 0, constraints)
}
