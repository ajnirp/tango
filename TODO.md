A list of ideas and improvements to take this project further. I might not get around to any of them.

* Pass around slices instead of references to vectors? More idiomatic?
* Look into implementing row and col iterators for the board Vec? [Reference](https://rust-for-c-programmers.com/ch13/13_3_creating_custom_iterators.html)
* Play around with 2D rendering libraries in Rust and draw these boards.
* Can Tango be generalized to higher dimensions? 3D at least?
* Can I write a Tango puzzle generator? Can I go a step further and allow the user to author a pattern of grey cells, and then I generate a testcase from that?
  * The key requirement is there should be exactly one solution.
  * Should be straightforward to extend the solver to check if there is more than one solution to a given testcase.
* Deploy this solver online. Use WASM? Provide a nice graphical interface for people to construct puzzles.