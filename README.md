# Classical Sum-check Protocol Benchmark

A Rust project implementing and benchmarking the classical Sum-check protocol over multilinear polynomials.

## Overview

This project implements the prover and verifier workflow of the classical Sum-check protocol and benchmarks its execution time for different polynomial sizes.

The project uses selected components from the [Plonky3](https://github.com/Plonky3/Plonky3) ecosystem, organized locally for experimentation and benchmarking.

## Features

* Classical Sum-check protocol
* Multilinear polynomials represented by evaluation tables
* Finite-field arithmetic over `F17`
* Prover and verifier workflows
* Random challenges in the verification process
* Criterion-based benchmarking

## Project Structure

.
├── benches/
   
│   ├── sumcheck_full.rs
   
│   └── sumcheck_prover.rs

├── crates/
   
│   ├── challenger/
   
│   ├── field/
  
│   ├── keccak/
   
│   ├── matrix/
   
│   ├── maybe-rayon/
   
│   ├── multilinear-util/
   
│   ├── symmetric/
   
│   └── util/

├── src/
  
│   ├── f17.rs

│   ├── lib.rs
  
│   ├── main.rs
   
│   ├── prover.rs
   
│   └── verifier.rs

├── tests/

├── Cargo.toml

└── Cargo.lock


The `crates/` directory contains selected crates from the Plonky3 project.

## Input

The program accepts the evaluation table of a multilinear polynomial over the Boolean hypercube.

The number of evaluations must be a non-zero power of two.

For example:


1 2 3 4


represents a multilinear polynomial with four evaluations.

The program computes the sum of the evaluations over the Boolean hypercube, the sum H with the given evaluations. 

## Building


cargo build


## Running


cargo run


The program reads the polynomial evaluation table from standard input and executes the corresponding prover/verifier workflow.

## Benchmarking

The project uses [Criterion](https://github.com/bheisler/criterion.rs) for benchmarking.

Run the benchmarks with:


cargo bench


The benchmarks measure execution time for different polynomial sizes and include separate benchmarks for the full protocol and the prover.

## Reference

The project uses selected crates from the [Plonky3](https://github.com/Plonky3/Plonky3) project.
