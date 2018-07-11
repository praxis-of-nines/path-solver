Traveling Salesman Solver
======

This is a Rust command-line program to determine a good solution to the traveling salesman problem, using a genetic algorithm.  It currently reduces path length pretty well but can use optimizations to lower the range of solution quality.

Thank you to dunnker for this rust version of the Java implementation by Lee Jacobson https://github.com/dunnker/traveling-salesman


## Usage

Rust is a prerequisite of course.

```
>cargo build
```

To run the project enter:

```
>cargo run
```

## Todo List

- Change from roulette to tournament style selection
- Add an additional file for use as an Erlang NIF
- Adjust mutation rate and population size to be optimal for about 10-50 locations as a default
