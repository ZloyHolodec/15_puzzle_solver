# 15 puzzle solver

Do you hate 15 puzzle in video games as I am? When I faced again with this most boring type of puzzles in the **Fatal Frame mask of the lunar eclipse** I decided to make a Rust script which will solve it for me.

This algorithm is not perfect and sometimes not able to solve puzzle for a reasonable amount of time. It randomly depends from starting condition. So if it works more than a minute it better to change starting position by yourself and execute again. Usually it very simple to solve first 2 lines of puzzles by yourself. After that script can find the rest of the solution for a ~200ms on a good CPU.


Yes you need to change source code to input the end state and start state. I did it for myself to solve exact puzzle in exact videogame so do not expect a high quality and user friendliness :).


How to execute script:
1. Open the `src/main.rs`
1. Find the main function
1. The `initial_state` is what you want to have at the end. This is a puzzle solution.
1. The `end_state` is what you have at the screen right now. Current state of the puzzle.

The 00 value is an empty block.

After you change code run the code via `cargo run --release`. You need to have installed Rust compiler on your system of course.