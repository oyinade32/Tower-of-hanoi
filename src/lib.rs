// Tower of Hanoi implementations

// Recursive solution
// Time complexity: O(2^n)
// Space complexity: O(n) due to recursion stack
pub fn solve_recursive(
    n: u32,
    from: u32,
    to: u32,
    helper: u32,
    moves: &mut Vec<(u32, u32)>
) {
    if n == 0 {
        return;
    }

    if n == 1 {
        moves.push((from, to));
        return;
    }

    solve_recursive(n - 1, from, helper, to, moves);
    moves.push((from, to));
    solve_recursive(n - 1, helper, to, from, moves);
}


// Iterative solution (for benchmarking comparison)
// Avoids recursion by using a pattern-based approach
// Time complexity: O(2^n)
// Space complexity: O(2^n) for storing moves
pub fn solve_iterative(n: u32) -> Vec<(u32, u32)> {
    let mut moves = Vec::new();

    let total_moves = (1 << n) - 1;

    let (src, aux, dest) = (1, 2, 3);

    for i in 1..=total_moves {
        if i % 3 == 1 {
            moves.push((src, dest));
        } else if i % 3 == 2 {
            moves.push((src, aux));
        } else {
            moves.push((aux, dest));
        }
    }

    moves
}