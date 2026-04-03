// GitHub: https://github.com/oyinade32/Tower-of-hanoi
// CSES Problem 2165: Tower of Hanoi

use std::io;

fn solve(n: u32, from: u32, to: u32, helper: u32) {
    if n == 1 {
        println!("{} {}", from, to);
        return;
    }

    solve(n - 1, from, helper, to);
    println!("{} {}", from, to);
    solve(n - 1, helper, to, from);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    let moves = (1 << n) - 1;
    println!("{}", moves);

    solve(n, 1, 3, 2);
}