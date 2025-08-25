// TODO: pass around slices instead of references to vectors?
// TODO: fix the "three identical" logic in "can_set"
// TODO: look into implementing row and col iterators for the board Vec?
// https://rust-for-c-programmers.com/ch13/13_3_creating_custom_iterators.html

mod board;
mod constraint;

use board::{at, col, inside, row, side};

fn can_set(board: &Vec<u8>, i: usize, constraints: &Vec<u8>, new: u8) -> bool {
    let (drs, dcs) = ([0usize, 1], [1usize, 0]);
    let side = side(&board);
    let (r, c) = (row(i, side), col(i, side));

    for v in 0..2 {
        let (dr, dc) = (drs[v], dcs[v]);
        let (mut _r, mut _c) = (r * dc, c * dr);

        let (mut streak, mut prev) = (1, 255u8);

        // number of cells in the row/col that are already equal to `new`
        let mut num_existing = 0;

        for _ in 0..side {
            let curr = if (_r, _c) == (r, c) {
                new
            } else {
                at(board, _r, _c)
            };

            if curr == new {
                num_existing += 1;
            }

            streak = if curr != 2 && curr == prev {
                streak + 1
            } else {
                1
            };
            // three consecutive identical not allowed
            if streak == 3 {
                return false;
            }

            prev = curr;
            _r += dr;
            _c += dc;
        }
        // no more than `side / 2` in a row or column
        // for example, for a 8x8 board, no more than 4 in a row or column
        if num_existing > side / 2 {
            return false;
        }
    }

    // TODO: why doesn't this work
    // for v in 0..2 {
    //     let (dr, dc) = (drs[v], dcs[v]);
    //     let (mut _r, mut _c) = if dr == 0 { (r, 0) } else { (0, c) };
    //     let (mut one, mut two) = (255u8, 255u8);
    //     for _ in 0..side {
    //         let three = if (_r, _c) == (r, c) { new } else { at(board, _r, _c) };
    //         if one == two && two == three {
    //             return false;
    //         }
    //         one = two;
    //         two = three;
    //         _r += dr;
    //         _c += dc;
    //     }
    // }

    // abide by constraints
    // north, east, south, west
    let _drs = [-1i16, 0, 1, 0];
    let _dcs = [0i16, 1, 0, -1];
    let constraint = at(constraints, r, c);
    for j in 0..8 {
        let rule = constraint & (1 << j);
        if rule == 0 {
            continue;
        }
        let _nr = (r as i16) + _drs[j % 4];
        let _nc = (c as i16) + _dcs[j % 4];
        if _nr < 0 || _nc < 0 {
            continue;
        }
        let (nr, nc) = (_nr as usize, _nc as usize);
        if !inside(&board, nr, nc) {
            continue;
        }
        let nbr_val = at(board, nr, nc);
        if nbr_val == 2 {
            continue;
        }
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
        return helper(board, i + 1, constraints);
    }
    for new in 0..2 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use board::parse_board;
    use constraint::{parse_constraints, Constraint};

    struct Testcase {
        board: &'static str,
        constraints: Vec<Constraint>,
        solution: &'static str,
    }

    fn print(board: &Vec<u8>) {
        let side = side(&board);
        for i in 0..side {
            for j in 0..side {
                print!("{} ", board[i * side + j]);
            }
            println!("");
        }
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

    fn solved(board: &Vec<u8>, constraints: &Vec<Constraint>) -> bool {
        valid(&board, &constraints) && board.iter().all(|&x| x != 2)
    }

    #[test]
    fn solver_works() {
        let testcases = vec![
            Testcase {
                board: "222222222222220022221022221122220022",
                constraints: vec![
                    Constraint {
                        from: 2,
                        to: 3,
                        eq: false,
                    },
                    Constraint {
                        from: 6,
                        to: 7,
                        eq: true,
                    },
                    Constraint {
                        from: 10,
                        to: 11,
                        eq: false,
                    },
                    Constraint {
                        from: 12,
                        to: 18,
                        eq: false,
                    },
                    Constraint {
                        from: 17,
                        to: 23,
                        eq: true,
                    },
                    Constraint {
                        from: 24,
                        to: 30,
                        eq: false,
                    },
                    Constraint {
                        from: 29,
                        to: 35,
                        eq: false,
                    },
                ],
                solution: "100101001101110010011010001101110010",
            },
            Testcase {
                board: "222222220022202212212202221122222222",
                constraints: vec![
                    Constraint {
                        from: 0,
                        to: 1,
                        eq: true,
                    },
                    Constraint {
                        from: 4,
                        to: 5,
                        eq: false,
                    },
                    Constraint {
                        from: 0,
                        to: 6,
                        eq: true,
                    },
                    Constraint {
                        from: 5,
                        to: 11,
                        eq: true,
                    },
                    Constraint {
                        from: 24,
                        to: 30,
                        eq: false,
                    },
                    Constraint {
                        from: 30,
                        to: 31,
                        eq: true,
                    },
                    Constraint {
                        from: 29,
                        to: 35,
                        eq: false,
                    },
                    Constraint {
                        from: 34,
                        to: 35,
                        eq: false,
                    },
                ],
                solution: "001101010011101010110100001101110010",
            },
            Testcase {
                board: "212222102222222222222222222210222212",
                constraints: vec![
                    Constraint {
                        from: 3,
                        to: 9,
                        eq: false,
                    },
                    Constraint {
                        from: 14,
                        to: 15,
                        eq: false,
                    },
                    Constraint {
                        from: 16,
                        to: 17,
                        eq: false,
                    },
                    Constraint {
                        from: 18,
                        to: 19,
                        eq: false,
                    },
                    Constraint {
                        from: 20,
                        to: 21,
                        eq: true,
                    },
                    Constraint {
                        from: 26,
                        to: 32,
                        eq: true,
                    },
                ],
                solution: "011001100110011001101100100110010011",
            },
            Testcase {
                board: "222222222222222202222120222020222212",
                constraints: vec![
                    Constraint {
                        from: 0,
                        to: 1,
                        eq: true,
                    },
                    Constraint {
                        from: 2,
                        to: 8,
                        eq: true,
                    },
                    Constraint {
                        from: 6,
                        to: 12,
                        eq: true,
                    },
                    Constraint {
                        from: 14,
                        to: 20,
                        eq: false,
                    },
                    Constraint {
                        from: 18,
                        to: 19,
                        eq: true,
                    },
                ],
                solution: "110100010011001101110100101010001011",
            },
            Testcase {
                board: "2222222020012221222222202100222222221102022222221222101212222222",
                constraints: vec![
                    Constraint {
                        from: 5,
                        to: 13,
                        eq: false,
                    },
                    Constraint {
                        from: 13,
                        to: 21,
                        eq: true,
                    },
                    Constraint {
                        from: 42,
                        to: 50,
                        eq: false,
                    },
                    Constraint {
                        from: 50,
                        to: 58,
                        eq: true,
                    },
                ],
                // solution: "0110011010011001001101101100100101101100001100111100101010010101",
                solution: "0110101010010101001101101100100101101100001100111100101010010101",
            },
        ];

        for (index, testcase) in testcases.iter().enumerate() {
            let (mut board, mut solution, mut constraints) = (
                vec![0u8; testcase.board.len()],
                vec![0u8; testcase.board.len()],
                vec![0u8; testcase.board.len()],
            );
            parse_board(&testcase.board, &mut board);
            parse_board(&testcase.solution, &mut solution);
            parse_constraints(&testcase.constraints, &mut constraints, side(&board));

            println!("Testcase {}", index + 1);
            assert_eq!(solve(&mut board, &constraints), true);
            if side(&board) == 8 {
                print(&board);
            }
            assert_eq!(solved(&board, &testcase.constraints), true);
            assert_eq!(board, solution);
        }
    }
}
