# tsp

A genetic algorithms framework written in Rust.

# Building the project

After cloning the repository, run `cargo build` to build the code. You can also use `cargo run --example <example name>` to run one of the included examples:

- `tsp`: an example for the Travelling Salesman Problem

# Genetic algorithms

Genetic algorithms provide a way of using methods loosely based on the concepts of evolution to approximate solutions to problems that would be otherwise difficult to solve. All problems defined with this framework needs to specify a fitness evaluation function that will be used to determine how good each solution is. In addition, it provides the following customization points:

## Selection

This determines how we select the individuals used for reproduction. The available selections are:

- Tournament

## Crossover

The method of sexual recombination. It takes two parent chromosomes and creates one or two children based on them. The available crossovers are:

- Cut and splice
- Cycle
- Edge recombination
- Half uniform
- 1-point
- Order 1
- Partially Mapped Crossover
- Precedence Preservative
- Three Parent
- 2-point
- Uniform

## Mutation

A method of asexual reproduction. It takes a parent chromosome and changes it slightly to create a new child. The available mutations are:

- Centre inverse
- Flip bit
- Partial shuffle
- Reverse sequence
- Shuffle
- Thrors
- Twors
- Uniform

## Reinsertion

After the new children were created they need to be put back into the population. Reinsertion allows the customization of how this happens, using the following methods:

- Elitist
- Fitness-based
- Pure reinsert
- Uniform

## Termination

The genetic algorithm is run until some termination condition is met. The available ones are:

- Fitness convergence
- Fitness threshold
- Iterations

## Tracking

By default, only the latest population is available at the end of the algorithm, however it is possible to add a tracking method that can record chromosomes accross iterations. The built-in ones are:

- Best solution
