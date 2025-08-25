// Boards are stored as arrays of `side*side` ints. `side` is the number of
// in one side of the square board.
// 0 = sun, 1 = moon, 2 = blank square.

pub fn side(board: &Vec<u8>) -> usize {
    board.len().isqrt()
}

pub fn row(i: usize, side: usize) -> usize {
    i / side
}

pub fn col(i: usize, side: usize) -> usize {
    i % side
}

pub fn at(board: &Vec<u8>, r: usize, c: usize) -> u8 {
    board[r * side(&board) + c]
}

pub fn inside(board: &Vec<u8>, r: usize, c: usize) -> bool {
    r * side(&board) + c < board.len()
}

// Expects `result` to already have the right size.
pub fn parse_board(input: &str, result: &mut Vec<u8>) {
    for (i, c) in input.chars().enumerate() {
        result[i] = c.to_digit(10).unwrap() as u8;
    }
}
