# Sudoku creator

Simple Rust implementation to create and solve sudoku riddles
with simple straightforward backtrack algorithm
inspired by [Java Sudoku](https://github.com/sfuhrm/sudoku)

Allow using alternative solver impls e.g. https://github.com/emerentius/sudoku
using the string format as input/output format

Possible to be made no_std (alloc required)
Then needs alternate rand impl and custom solvers are limited