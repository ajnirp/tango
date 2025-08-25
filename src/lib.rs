// TODO: pass around slices instead of references to vectors?
// TODO: fix the "three identical" logic in "can_set"
// TODO: look into implementing row and col iterators for the board Vec?
// https://rust-for-c-programmers.com/ch13/13_3_creating_custom_iterators.html

mod board;
mod constraint;
mod solver;
mod testcases;
mod testing;

pub use board::parse_board;
pub use constraint::{parse_constraints, Constraint};
pub use solver::solve;
pub use testcases::testcases;
pub use testing::{is_solved, Testcase};

#[cfg(test)]
mod tests {
    use crate::board::{parse_board, side};
    use crate::constraint::parse_constraints;
    use crate::solver::solve;
    use crate::testcases::testcases;
    use crate::testing::is_solved;

    #[test]
    fn solver_works() {
        for testcase in testcases().iter() {
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
