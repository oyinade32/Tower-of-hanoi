use std::time::Instant;
use tower_of_hanoi::{solve_recursive, solve_iterative};

fn main() {
    let n = 20;

    // -------------------------
    // Recursive benchmark
    // -------------------------
    let mut moves_recursive = Vec::new();

    let start = Instant::now();
    solve_recursive(n, 1, 3, 2, &mut moves_recursive);
    let duration_recursive = start.elapsed();

    // -------------------------
    // Iterative benchmark
    // -------------------------
    let start = Instant::now();
    let moves_iterative = solve_iterative(n);
    let duration_iterative = start.elapsed();

    // -------------------------
    // Results
    // -------------------------
    println!("n = {}", n);

    println!(
        "Recursive: moves = {}, time = {:?}",
        moves_recursive.len(),
        duration_recursive
    );

    println!(
        "Iterative: moves = {}, time = {:?}",
        moves_iterative.len(),
        duration_iterative
    );
}