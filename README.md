# Tower of Hanoi (Rust Implementation)

## 📌 Overview

This project implements the **Tower of Hanoi** problem using recursion in Rust.
The goal is to move `n` disks from a source peg to a destination peg using a helper peg, following the rules:

* Only one disk can be moved at a time
* A larger disk cannot be placed on top of a smaller disk

---

## 🧠 Approach

The solution uses a **recursive algorithm**:

1. Move `n-1` disks from source to helper
2. Move the remaining disk to destination
3. Move the `n-1` disks from helper to destination

The moves are stored in a `Vec<(u32, u32)>` for:

* easier testing
* better modularity
* benchmarking support

---

## ⚙️ Implementation Details

* Language: Rust
* Core logic implemented in `lib.rs`
* Execution and output handled in `main.rs`
* Unit tests included for correctness verification

---

## 📊 Complexity Analysis

* **Time Complexity:** O(2^n)
* **Space Complexity:** O(2^n) (due to storing all moves)

---

## 🚀 Notes

* This implementation stores all moves in memory using a `Vec`.
* In competitive programming environments (e.g., CSES), printing moves directly is more memory-efficient.
* Recursion is used, but with constraint `n ≤ 16`, the recursion depth is safe and does not cause stack overflow.

---

## ▶️ How to Run

```bash
cargo run --bin tower_of_hanoi
```

---

## ✅ Testing

Run tests using:

```bash
cargo test
```

---

## 📁 Project Structure

```
benches/
 └── hanoi_benchmark.rs

src/
 ├── lib.rs
 └── main.rs

tests/
 └── hanoi_test.rs

Cargo.toml
README.md
.gitignore
```

---

## 👨‍💻 Author

Oyinade Olayinka
