# Rust Simple Exercises

A collection of simple Rust exercises for beginners. Each exercise is a separate Cargo project in a workspace, focusing on different Rust concepts.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (rustc, cargo)

## How to run

```bash
# From the workspace root (any exercise)
cargo run -p <folder_name>

# Or go into the exercise folder
cd <folder_name>
cargo run
```

## Exercises

| Folder | Description | Key concepts |
|---|---|---|
| `Hello_world` | Ask the user for their name and greet them | variables, mutability, String vs &str, stdin |
| `Fibonacci` | Compute the nth Fibonacci number recursively | recursion, functions |
| `Collatz_sequence` | Calculate the length of the Collatz sequence | loops, conditionals |
| `Simple_calculator` | Interactive calculator (+, -, *, /) | match, parsing, error handling |
| `Matrix_transposition` | Transpose a 3x3 matrix | 2D arrays, unit tests |
| `Rmanager` | Manage a shopping/article list (CRUD) | Vec, loops, basic CRUD |
| `todo` | Persistent todo list app with JSON storage | serde, serialization, enums, file I/O |
