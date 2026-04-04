// In this implementation, a Vec is used to store moves for testing and benchmarking purposes.
// This makes the solution more modular and easier to verify.
// However, for competitive programming (e.g., CSES), printing moves directly
// would be more memory efficient, since it avoids storing all moves in memory.



pub fn solve(n: u32, from: u32, to: u32, helper: u32, moves: &mut Vec<(u32, u32)>) {
    if n == 0 {
        return;
    }

    if n == 1 {
        moves.push((from, to));
        return;
    }

    solve(n - 1, from, helper, to, moves);
    moves.push((from, to));
    solve(n - 1, helper, to, from, moves);
}

// Note: This solution uses recursion, which incurs function call stack overhead.
// However, since the constraint n ≤ 16, the recursion depth is small and safe.