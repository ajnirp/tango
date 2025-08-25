# About

* A library implementing a backtracking solver for LinkedIn Tango.
* Code might not be very idiomatic.
* No significant optimizations used.
* Some testcases included.

# Running the code

`cargo test`

# Usage

The `tests` mod in `lib.rs` contains examples of how to specify a testcase, and run the solver on a testcase.

# How the solver works

It uses a simple backtracking algorithm. For each unfilled square:

* See if you can place a `0` there without violating:
  * the requirement that no three cells contain the same value
  * the requirement that no row or column has more than half of the cells set to the same value
  * the equality / inequality constraints
* If yes, set the value and make a recursive call to the solver with the next index.
  * If the recursive call succeeds, we're done.
  * If the recursive call fails, we can't place a `0` there.
* Now try the above but with a `1`.
* If neither works, we can't place either value there. Our solution thus far won't work. Return `false`.
