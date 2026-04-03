use tower_of_hanoi::solve;
use std::time::Instant;

fn run_benchmark(n: u32) {
    let mut moves = Vec::new();

    let start = Instant::now();
    solve(n, 1, 3, 2, &mut moves);
    let duration = start.elapsed();

    println!("n = {}, moves = {}, time = {:?}", n, moves.len(), duration);
}

fn main() {
    run_benchmark(10);
    run_benchmark(15);
}