mod board;
mod constraint;
mod solver;
mod testcases;
mod testing;

pub use board::Board;
pub use constraint::Constraint;
pub use solver::solve;
pub use testcases::testcases;
pub use testing::{is_solved, Testcase};

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::constraint::Constraint;
    use crate::solver::solve;
    use crate::testcases::testcases;
    use crate::testing::is_solved;

    #[test]
    fn solver_works() {
        for testcase in testcases().iter() {
            let mut board = Board::parse_from(&testcase.board, &testcase.constraints);
            let solution = Board::parse_from(&testcase.solution, &Vec::<Constraint>::new());

            assert_eq!(solve(&mut board), true);
            assert_eq!(is_solved(&board, &testcase.constraints), true);
            assert_eq!(board, solution);
        }
    }
}
