# Tower of Hanoi (Rust Implementation)

## Overview

This project implements the **Tower of Hanoi** problem in Rust using two different approaches: a recursive solution and an iterative solution.

The objective is to move `n` disks from a source peg to a destination peg using a helper peg, following these rules:

* Only one disk can be moved at a time
* A larger disk cannot be placed on top of a smaller disk

---

## Implementations

### 1. Recursive Approach

The recursive solution follows the classical definition:

1. Move `n-1` disks from the source peg to the helper peg
2. Move the largest disk to the destination peg
3. Move the `n-1` disks from the helper peg to the destination peg

This approach is simple, elegant, and closely matches the mathematical definition of the problem. However, it uses the function call stack, which introduces some overhead.

---

### 2. Iterative Approach

An alternative **iterative solution** is implemented to allow meaningful performance comparison.

* Uses a loop-based method instead of recursion
* Avoids function call overhead
* Simulates the movement pattern without recursive calls

This version is useful for benchmarking and understanding how implementation style affects performance.

---

## Implementation Details

* Language: Rust
* Core logic: `src/lib.rs`
* Program execution: `src/main.rs`
* Benchmarking: `src/bin/hanoi_benchmark.rs`
* Testing: `tests/hanoi_test.rs`

Moves are stored in a `Vec<(u32, u32)>` to support testing and benchmarking.

---

##  Complexity Analysis

Both implementations have the same theoretical complexity:

* **Time Complexity:** O(2^n)
* **Space Complexity:**

  * Recursive: O(n) due to recursion stack
  * Iterative: O(2^n) due to storing moves

This is because the number of required moves is fixed at `2^n - 1`.

---

##  Benchmark Analysis

To evaluate performance, both implementations were benchmarked.

Although both algorithms have the same theoretical time complexity O(2^n), there are differences in actual runtime:

* The **recursive approach** introduces overhead due to repeated function calls and use of the call stack
* The **iterative approach** avoids this overhead by using a loop-based structure

As a result, the iterative version may perform slightly faster in practice, even though both execute the same number of operations.

This demonstrates the difference between **theoretical complexity** and **real-world performance**.

---

## Notes

* Storing moves in a `Vec` improves testability and modularity
* In competitive programming environments (e.g., CSES), printing moves directly is more memory-efficient
* With constraint `n ≤ 16`, recursion depth is safe and does not cause stack overflow

---

## How to Run

```bash
cargo run --bin tower_of_hanoi
```

---

## Run Benchmark

```bash
cargo run --bin hanoi_benchmark
```

---

## Testing

```bash
cargo test
```

---

## Project Structure

```
src/
 ├── lib.rs
 ├── main.rs
 └── bin/
     └── hanoi_benchmark.rs

tests/
 └── hanoi_test.rs

Cargo.toml
README.md
.gitignore
```

---

## Author

Oyinade Olayinka


