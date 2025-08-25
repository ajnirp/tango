// TODO: pass around slices instead of references to vectors?
// TODO: fix the "three identical" logic in "can_set"
// TODO: look into implementing row and col iterators for the board Vec?
// https://rust-for-c-programmers.com/ch13/13_3_creating_custom_iterators.html

mod board;
mod constraint;
mod solver;
mod testing;

pub use board::parse_board;
pub use constraint::{parse_constraints, Constraint};
pub use solver::solve;
pub use testing::{Testcase, is_solved};

#[cfg(test)]
mod tests {
    use crate::board::{parse_board, side};
    use crate::constraint::{parse_constraints, Constraint};
    use crate::solver::solve;
    use crate::testing::{Testcase, is_solved};

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
                solution: "0110101010010101001101101100100101101100001100111100101010010101",
            },
        ];

        for testcase in testcases.iter() {
            let (mut board, mut solution, mut constraints) = (
                vec![0u8; testcase.board.len()],
                vec![0u8; testcase.board.len()],
                vec![0u8; testcase.board.len()],
            );
            parse_board(&testcase.board, &mut board);
            parse_board(&testcase.solution, &mut solution);
            parse_constraints(&testcase.constraints, &mut constraints, side(&board));

            assert_eq!(solve(&mut board, &constraints), true);
            assert_eq!(is_solved(&board, &testcase.constraints), true);
            assert_eq!(board, solution);
        }
    }
}
