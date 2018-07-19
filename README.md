Traveling Salesman Solver
======

This is a Rust command-line program to determine a good solution to the traveling salesman problem, using a genetic algorithm.  It currently reduces path length pretty well but can use optimizations to lower the range of solution quality.

Thank you to dunnker for creating the original rust version of the Java implementation by Lee Jacobson that is modified to create my program (https://github.com/dunnker/traveling-salesman)


## Usage

Rust is a prerequisite of course.

```
> cargo build
```

To run the project enter (with command arguments of nodes to visit):

```
> cargo run 100,28 672,72 2220,56 1096,444 1084,864 2228,944 2528,1052 2164,1212 2276,1468
```

To run the project in production mode (MUCH faster)

```
> cargo build --release
> ./target/release/path_solver 100,28 672,72 2220,56 1096,444 1084,864 2228,944 2528,1052 2164,1212 2276,1468 696,1544 1020,1812 8,2136 612,2084 732,2320 896,2500 2668,2608 200,2440 20,20 60,20 1200,2090 1250,2455 20,280 3,2090
```

## Todo List

- COMPLETE Allow tournament style selection to compare with roulette (currently getting outperformed. can improve)
- Add an additional file for use as an Erlang NIF entry
- Adjust mutation rate and population size to be optimal for about 10-50 locations as a default, or make it dynamic
